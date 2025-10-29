extends Control

# ------------------------------------------------------------------------------
# --- CHUNK & RENDER CONSTANTS ---
# ------------------------------------------------------------------------------
const CHUNK_SIZE_X: int = 64
const CHUNK_SIZE_Y: int = 64
const DEFAULT_CHUNKS: int = 12

# 🎯 THROTTLING CONSTANTS
const MAX_CHUNKS_PER_FRAME: int = 5
const MAX_QUEUE_SIZE: int = 150 # Critical for backpressure

# ------------------------------------------------------------------------------
# 🧭 UI Node References
# ------------------------------------------------------------------------------
@onready var grid_width: SpinBox = $gridwidthspinbox
@onready var grid_height: SpinBox = $gridheightspinbox
@onready var seed_input: LineEdit = $seedlineedit
@onready var placement_mode_selector: OptionButton = $placementoptionbutton
@onready var tile_type_selector: OptionButton = $tiletypeoptionbutton
@onready var animate_checkbox: CheckBox = $animatecheckbox
@onready var status_label: Label = $billboard
@onready var generate_button: Button = $ignition
@onready var progress_bar: ProgressBar = $progressbar
@onready var toggle_terminal_button: Button = $toggleterminalbutton
@onready var engine_timer: Timer = $enginetimer
@onready var engine_timer_label: Label = $enginetimerlabel
@onready var tiles_placed_label: Label = $tilesplacedlabel
@onready var tile_placement_time_label: Label = $tiletimeofplacement # ⬅️ NEW REFERENCE

# 🧠 External Scene References - Use explicit paths for non-siblings
@onready var main: Node2D = get_parent() as Node2D
@onready var ssxl_engine: Node = main.get_node("SSXLEngine")
@onready var ssxl_signals: Node = main.get_node("SSXLSignals")
@onready var expansive_tilemap: TileMap = main.get_node("expansive_tilemap") as TileMap
@onready var clock_label: Label = main.get_node("tilemap/clocklabel") as Label
@onready var clock_timer: Timer = main.get_node("tilemap/clocktimer") as Timer
@onready var tilemap_node: Node2D = main.get_node("tilemap") as Node2D
@onready var cameras: Node = main.get_node("cameras")
@onready var camera1: Camera2D = main.get_node("cameras/camera1") as Camera2D
@onready var camera2: Camera2D = main.get_node("cameras/camera2") as Camera2D

# 📊 State Variables
var last_percent: int = -1
var tile_size: Vector2 = Vector2(16, 16)
var panel_collapsed: bool = false
var engine_tick_count: int = 0
var total_chunks_processed: int = 0
var total_tiles_placed: int = 0
var is_generating: bool = false
var initial_zoom_set: bool = false
var current_camera_id: int = 1 # 1: UI, 2: Map
var generation_start_time_ms: int = 0 # ⬅️ NEW: Store start time in milliseconds

# 🚀 BATCHING: Arrays for high-performance updates
var cells_to_update: Array[Vector2i] = []
var chunks_to_process_queue: Array[Vector2i] = []


# ------------------------------------------------------------------------------
# 🧭 BOOT SEQUENCE
# ------------------------------------------------------------------------------
func _ready() -> void:
	_setup_timers()

	# Critical Node Validation
	if not is_instance_valid(expansive_tilemap):
		push_error("❌ FATAL: Initialization Error: expansive_tilemap is missing or invalid.")
		status_label.text = "❌ FATAL: TileMap missing."
		return

	_setup_engine_links()
	_setup_ui()
	_connect_signals()
	
	# Initial camera switch
	if cameras and cameras.has_method("switch_to_camera"):
		cameras.switch_to_camera(current_camera_id)

	# Get canonical tile size from the TileSet
	var tileset: TileSet = expansive_tilemap.get_tileset()
	if tileset:
		tile_size = tileset.get_tile_size()

func _setup_timers() -> void:
	engine_timer.wait_time = 1.0
	engine_timer.one_shot = false
	engine_timer.autostart = false
	engine_timer.timeout.connect(_on_engine_timer_timeout)
	
	if clock_timer:
		clock_timer.timeout.connect(_on_clock_timer_timeout)

func _setup_engine_links() -> void:
	# Link up the necessary resources with the SSXLEngine
	if ssxl_engine:
		if ssxl_signals and ssxl_engine.has_method("set_signals_node"):
			ssxl_engine.set_signals_node(ssxl_signals)
		if expansive_tilemap and ssxl_engine.has_method("set_tilemap"):
			ssxl_engine.set_tilemap(expansive_tilemap)
	else:
		push_error("❌ SSXLEngine not found.")


func _setup_ui() -> void:
	grid_width.max_value = 1_000_000_000.0
	grid_height.max_value = 1_000_000_000.0
	grid_width.step = float(CHUNK_SIZE_X)
	grid_height.step = float(CHUNK_SIZE_Y)
	
	# Set default grid to 768 x 768 (12 chunks * 64)
	grid_width.value = float(CHUNK_SIZE_X * DEFAULT_CHUNKS)
	grid_height.value = float(CHUNK_SIZE_Y * DEFAULT_CHUNKS)

	placement_mode_selector.clear()
	placement_mode_selector.add_item("perlin_basic_2d")
	placement_mode_selector.add_item("cellular_automata_checkerboard")
	placement_mode_selector.add_item("cellular_automata_basic")
	placement_mode_selector.add_item("cellular_automata_maze")
	placement_mode_selector.add_item("cellular_automata_solid")
	placement_mode_selector.select(0)

	tile_type_selector.clear()
	tile_type_selector.add_item("Basic")
	tile_type_selector.add_item("Automata")
	tile_type_selector.select(0)

	progress_bar.min_value = 0.0
	progress_bar.max_value = 100.0
	progress_bar.value = 0.0
	progress_bar.visible = false

	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: 0"
	
	if is_instance_valid(tile_placement_time_label): # ⬅️ INITIALIZE NEW LABEL
		tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"

	status_label.text = "🟢 Ready to generate."


func _connect_signals() -> void:
	generate_button.pressed.connect(_on_generate_pressed)
	toggle_terminal_button.pressed.connect(_on_toggle_terminal_button_pressed)

	if ssxl_engine and not ssxl_engine.status_updated.is_connected(_on_engine_status_updated):
		ssxl_engine.status_updated.connect(_on_engine_status_updated)

	if ssxl_signals:
		if ssxl_signals.has_signal("build_map_start") and not ssxl_signals.build_map_start.is_connected(_on_build_map_start):
			ssxl_signals.build_map_start.connect(_on_build_map_start)
		
		if ssxl_signals.has_signal("chunk_generated") and not ssxl_signals.chunk_generated.is_connected(_on_chunk_generated):
			ssxl_signals.chunk_generated.connect(_on_chunk_generated)
			
		if ssxl_signals.has_signal("build_map_complete") and not ssxl_signals.build_map_complete.is_connected(_on_build_map_complete_received):
			ssxl_signals.build_map_complete.connect(_on_build_map_complete_received)
			
		if ssxl_signals.has_signal("generation_error") and not ssxl_signals.generation_error.is_connected(_on_generation_error):
			ssxl_signals.generation_error.connect(_on_generation_error)

# ------------------------------------------------------------------------------
# 🎮 INPUT HANDLING (FIX for Space Bar)
# ------------------------------------------------------------------------------
func _input(event: InputEvent) -> void:
	# Only process key down events that are not consumed by UI elements
	if event is InputEventKey and event.is_pressed() and not event.is_echo():
		if event.keycode == KEY_SPACE:
			var focused_control: Control = get_viewport().gui_get_focus_owner()
			
			# Ignore Space if typing in an input field
			if focused_control is LineEdit or focused_control is SpinBox:
				return

			# Toggle camera if generating or finished (button not disabled)
			if is_generating or not generate_button.disabled:
				_toggle_camera()
				get_viewport().set_input_as_handled()
				
# ------------------------------------------------------------------------------
# 🔄 ENGINE LOOP & THROTTLED PROCESSING
# ------------------------------------------------------------------------------
func _process(_delta: float) -> void:
	if not is_generating:
		return
		
	# 🎯 THROTTLING LOGIC
	var chunks_processed_this_frame: int = 0

	while not chunks_to_process_queue.is_empty() and chunks_processed_this_frame < MAX_CHUNKS_PER_FRAME:
		var coords: Vector2i = chunks_to_process_queue.pop_front()
		_process_chunk_data(coords.x, coords.y)
		chunks_processed_this_frame += 1
	
	# 🚀 BATCH RENDER
	if not cells_to_update.is_empty():
		expansive_tilemap.notify_runtime_tile_data_update(0)
		cells_to_update.clear()

	# 🟢 BACKPRESSURE RELEASE
	if chunks_to_process_queue.size() < MAX_QUEUE_SIZE / 2:
		if ssxl_signals and ssxl_signals.has_signal("chunk_generated") and not ssxl_signals.chunk_generated.is_connected(_on_chunk_generated):
			ssxl_signals.chunk_generated.connect(_on_chunk_generated)
			# print("🟢 Backpressure: Reconnected chunk_generated signal. Queue size: %d" % chunks_to_process_queue.size())

	# Update Engine Status Label
	if ssxl_engine and ssxl_engine.has_method("get_status"):
		var status: String = ssxl_engine.call("get_status")
		status_label.text = "🧠 Engine Status: %s (Queue: %d)" % [status, chunks_to_process_queue.size()]


# ------------------------------------------------------------------------------
# 🚀 GENERATION START & END
# ------------------------------------------------------------------------------
func _on_generate_pressed() -> void:
	# 🛑 GUARD
	if is_generating:
		print("⚠️ Generation already in progress. Ignoring button press.")
		return
		
	if not ssxl_engine or not ssxl_signals or not is_instance_valid(expansive_tilemap):
		push_error("Critical external node missing. Cannot start generation.")
		return

	# Input gathering
	var width: int = int(grid_width.value)
	var height: int = int(grid_height.value)
	var generator_name: String = placement_mode_selector.get_item_text(placement_mode_selector.selected)

	# Seed handling
	var seed: int
	if seed_input.text.is_valid_int():
		seed = int(seed_input.text)
	else:
		seed = randi() % 1_000_000
		seed_input.text = str(seed)
		status_label.text = "⚠️ Invalid seed. Using random seed: %d" % seed
		
	if width <= 0 or height <= 0 or (float(width) / CHUNK_SIZE_X) * (float(height) / CHUNK_SIZE_Y) > 10_000.0:
		status_label.text = "⚠️ Invalid grid size. Max chunks limit exceeded."
		return

	# State Setup: CLEAR counters for new generation
	_clear_generation_state()
	is_generating = true
	initial_zoom_set = false

	# ⬅️ NEW: Record the exact start time before the FFI call
	generation_start_time_ms = Time.get_ticks_msec() 

	# Max value calculation (Total Tiles in the map)
	var total_chunks_x: float = ceil(float(width) / CHUNK_SIZE_X)
	var total_chunks_y: float = ceil(float(height) / CHUNK_SIZE_Y)
	progress_bar.max_value = float(total_chunks_x * total_chunks_y * CHUNK_SIZE_X * CHUNK_SIZE_Y)
	progress_bar.value = 0.0
	progress_bar.visible = true
	
	status_label.text = "🧬 Generating map with mode: %s..." % [generator_name]
	generate_button.disabled = true
	engine_timer.start()

	# Clear map and force update before FFI call
	expansive_tilemap.clear()
	expansive_tilemap.emit_signal("tile_data_changed", 0, Vector2i.ZERO, Vector2i(width, height))
	cells_to_update.clear()
	await get_tree().process_frame

	# Camera switch to map view
	if cameras and cameras.has_method("switch_to_camera"):
		current_camera_id = 2
		cameras.switch_to_camera(current_camera_id)

	# FFI CALL: Initiates the asynchronous generation loop in Rust
	ssxl_engine.build_map(width, height, str(seed), generator_name)
	print("🧪 ControlPanel: build_map called with seed %d" % seed)

# --- NEW WRAPPER TO DEFER COMPLETION ---
func _on_build_map_complete_received() -> void:
	print("📡 Signal: build_map_complete received. Waiting for queue to clear...")
	
	await get_tree().process_frame
	
	# Wait until all chunks are processed
	while not chunks_to_process_queue.is_empty():
		await get_tree().process_frame
		print("⏱️ Waiting for queue to clear... remaining: %d" % chunks_to_process_queue.size())
		
	_on_build_map_complete()
	
func _on_build_map_complete() -> void:
	print("✅ Finalizing map generation.")
	progress_bar.visible = false
	
	var elapsed_engine_time: float = engine_timer.wait_time - engine_timer.time_left
	
	# ⬅️ NEW: Calculate the real-world elapsed time for tile placement
	var elapsed_placement_time_ms: int = Time.get_ticks_msec() - generation_start_time_ms
	var elapsed_placement_time_sec: float = float(elapsed_placement_time_ms) / 1000.0
	
	var final_tile_count: int = total_tiles_placed
	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: %d" % final_tile_count
		
	# ⬅️ NEW: Update the new label
	if is_instance_valid(tile_placement_time_label):
		tile_placement_time_label.text = "⏱️ Tile Placement Time: %.2fs" % elapsed_placement_time_sec
		
	engine_timer_label.text = "⏱️ Final Runtime: %.2fs | Ticks: %d" % [elapsed_engine_time, engine_tick_count]

	# Final camera positioning and zoom
	if camera2:
		var map_width_tiles: int = int(grid_width.value)
		var map_height_tiles: int = int(grid_height.value)
		
		var full_map_width: float = float(map_width_tiles) * tile_size.x
		var full_map_height: float = float(map_height_tiles) * tile_size.y

		camera2.global_position = Vector2(full_map_width / 2.0, full_map_height / 2.0)
		
		var viewport_size: Vector2 = get_viewport_rect().size
		var zoom_factor: float = min(viewport_size.x / full_map_width, viewport_size.y / full_map_height) * 0.9
		
		camera2.zoom = Vector2(clampf(zoom_factor, 0.05, 1.0), clampf(zoom_factor, 0.05, 1.0))
		initial_zoom_set = true

	# CRITICAL SYNCHRONIZATION: Ensure the last batch is processed and render state is clean
	if not cells_to_update.is_empty():
		expansive_tilemap.notify_runtime_tile_data_update(0)
	
	# Final full map redraw.
	expansive_tilemap.emit_signal("tile_data_changed", 0, Vector2i.ZERO, Vector2i(int(grid_width.value), int(grid_height.value)))
	cells_to_update.clear()
	
	await get_tree().process_frame

	_reset_temporary_state()
	status_label.text = "✅ Generation Complete. Map built. (%d tiles)" % final_tile_count


# ------------------------------------------------------------------------------
# 📡 SIGNAL HANDLERS
# ------------------------------------------------------------------------------
func _on_engine_status_updated(status_message: String) -> void:
	if status_message.begins_with("GENERATING:") or status_message.begins_with("ERROR:") or status_message.begins_with("IDLE:"):
		status_label.text = status_message

func _on_build_map_start() -> void:
	print("📡 Signal: build_map_start")

func _on_chunk_generated(chunk_x: int, chunk_y: int) -> void:
	chunks_to_process_queue.append(Vector2i(chunk_x, chunk_y))
	
	# Backpressure logic
	if chunks_to_process_queue.size() >= MAX_QUEUE_SIZE:
		if ssxl_signals.chunk_generated.is_connected(_on_chunk_generated):
			ssxl_signals.chunk_generated.disconnect(_on_chunk_generated)
			print("🛑 Backpressure: Disconnected chunk_generated signal. Queue size: %d" % MAX_QUEUE_SIZE)

func _on_generation_error(error_message: String) -> void:
	print("❌ ERROR: Generation failed: %s" % error_message)
	_reset_temporary_state()
	status_label.text = "❌ ERROR: Generation failed. Check console."


# ------------------------------------------------------------------------------
# --- CRITICAL: DEFERRED CHUNK PROCESSING ---
# ------------------------------------------------------------------------------
func _process_chunk_data(chunk_x: int, chunk_y: int) -> void:
	
	if not is_instance_valid(ssxl_engine):
		push_error("SSXLEngine is invalid. Stopping generation.")
		_reset_temporary_state()
		return

	# 1. Retrieve Chunk Data
	var chunk_dict: Variant = ssxl_engine.generate_chunk(chunk_x, chunk_y, 0)
	
	# 2. Validation Checks
	if typeof(chunk_dict) != TYPE_DICTIONARY:
		push_error("SSXLEngine.generate_chunk() returned invalid type: %s. Chunk: (%d, %d)" % [typeof(chunk_dict), chunk_x, chunk_y])
		_reset_temporary_state()
		status_label.text = "❌ FATAL: Engine chunk data is corrupted (Invalid Type)."
		return
		
	var tile_array: Array = chunk_dict.get("tiles", [])
	var tiles_in_chunk: int = tile_array.size()

	if tiles_in_chunk == 0:
		progress_bar.value += float(CHUNK_SIZE_X * CHUNK_SIZE_Y)
		total_chunks_processed += 1
		return

	# 3. TileMap Setup Check
	const layer: int = 0
	const source_id: int = 0
	var tile_index: int = 0
	
	var tileset: TileSet = expansive_tilemap.get_tileset()
	var atlas_source: TileSetAtlasSource = tileset.get_source(source_id) if is_instance_valid(tileset) else null
	
	if atlas_source == null:
		push_error("TileMap source ID %d not found or TileSet is invalid." % source_id)
		_reset_temporary_state()
		return

	# 4. Iterate and Place Tiles
	var cells_in_chunk: Array[Vector2i] = []
	
	for tile_data_variant in tile_array:
		var tile_data: Dictionary = tile_data_variant
		var tile_id: int = tile_data.get("id", 0)
		
		var local_x: int = tile_index % CHUNK_SIZE_X
		var local_y: int = tile_index / CHUNK_SIZE_X
		
		var global_x: int = (chunk_x * CHUNK_SIZE_X) + local_x
		var global_y: int = (chunk_y * CHUNK_SIZE_Y) + local_y
		
		var tile_coords: Vector2i = Vector2i(global_x, global_y)
		var atlas_coords: Vector2i = Vector2i(tile_id, 0)

		if atlas_source.has_tile(atlas_coords):
			expansive_tilemap.set_cell(layer, tile_coords, source_id, atlas_coords, 0)
			cells_in_chunk.append(tile_coords)
		else:
			expansive_tilemap.erase_cell(layer, tile_coords)

		tile_index += 1
		
	# BATCH RENDER UPDATE QUEUE
	cells_to_update.append_array(cells_in_chunk)

	# 5. Update Trackers
	progress_bar.value += float(tiles_in_chunk)
	total_chunks_processed += 1
	
	total_tiles_placed += tiles_in_chunk
	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: %d" % total_tiles_placed
	
	var percent: int = int(progress_bar.value / progress_bar.max_value * 100.0)

	# THROTTLED STATUS UPDATE
	if percent != last_percent and (percent % 1 == 0 or percent == 100):
		status_label.text = "🏗️ Chunk (%d, %d) placed... %d%%" % [chunk_x, chunk_y, percent]
		
	last_percent = percent


# ------------------------------------------------------------------------------
# --- UTILITY AND MISC. ---
# ------------------------------------------------------------------------------
func _toggle_camera() -> void:
	if not cameras or not cameras.has_method("switch_to_camera"):
		return
		
	if current_camera_id == 1:
		current_camera_id = 2
	else:
		current_camera_id = 1
		
	cameras.switch_to_camera(current_camera_id)
	print("Camera toggled to: %d" % current_camera_id)

## Resets only the temporary state (flags, timers, queues) but PRESERVES total_tiles_placed and generation_start_time_ms.
func _reset_temporary_state() -> void:
	is_generating = false
	generate_button.disabled = false
	engine_timer.stop()
	chunks_to_process_queue.clear()
	cells_to_update.clear()
	
	# Only reset the engine tick counter
	engine_tick_count = 0

## Clears all generation-related state, including the tile counter and recorded time.
func _clear_generation_state() -> void:
	_reset_temporary_state()
	
	# Reset state and label
	total_tiles_placed = 0
	generation_start_time_ms = 0 # ⬅️ NEW: Reset start time
	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: 0"
	if is_instance_valid(tile_placement_time_label): # ⬅️ CLEAR PLACEMENT TIME
		tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"


func _on_clock_timer_timeout() -> void:
	clock_label.text = "🕒 " + Time.get_datetime_string_from_system()
	engine_tick_count += 1

func _on_toggle_terminal_button_pressed() -> void:
	panel_collapsed = !panel_collapsed

	# Hide/Show elements based on the panel_collapsed state
	for child in get_children():
		# Filter to only affect the UI controls we want to collapse
		if child != toggle_terminal_button and child is Control and child != status_label and child != engine_timer_label and child != tiles_placed_label and child != tile_placement_time_label: # ⬅️ ADD NEW LABEL
			child.visible = not panel_collapsed
	
	# Control visibility of external references
	if clock_label:
		clock_label.visible = not panel_collapsed
	if progress_bar:
		# Keep progress bar visible if generating, even if panel is collapsed
		progress_bar.visible = not panel_collapsed and is_generating
	
	if tilemap_node:
		tilemap_node.visible = not panel_collapsed
	

func _on_engine_timer_timeout() -> void:
	var elapsed: float = engine_timer.wait_time - engine_timer.time_left
	
	# Update label to show both runtime and ticks
	engine_timer_label.text = "⏱️ Runtime: %.2fs | Ticks: %d" % [elapsed, engine_tick_count]
	
	# FFI CALL: Sends a tick to the native engine if the method exists
	if ssxl_engine and ssxl_engine.has_method("tick"):
		ssxl_engine.tick(engine_tick_count)
