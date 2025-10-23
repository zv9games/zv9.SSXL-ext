extends Control

# --- CHUNK CONSTANTS ---
const CHUNK_SIZE_X = 64
const CHUNK_SIZE_Y = 64

# 🧭 UI Node References
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

# 🧠 External Scene References
@onready var main := get_node("../")
@onready var aetherion_engine := main.get_node("AetherionEngine")
@onready var aetherion_signals := main.get_node("AetherionSignals")
@onready var expansive_tilemap := main.get_node("expansive_tilemap")
@onready var clock_label := main.get_node("tilemap/clocklabel")
@onready var clock_timer := main.get_node("tilemap/clocktimer")
@onready var tilemap := main.get_node("tilemap")
# Cameras node is the script owner for the camera logic (cameras.gd)
@onready var cameras := main.get_node("cameras") 
# The camera nodes themselves are primarily controlled by the cameras script, but we keep a reference 
# to the map camera (camera2) for initial zoom setup.
@onready var camera1: Camera2D = main.get_node("cameras/camera1")
@onready var camera2: Camera2D = main.get_node("cameras/camera2")

# 📊 State Variables
var last_percent := -1
var tile_size := Vector2(16, 16)
var panel_collapsed := false
var engine_tick_count := 0
var total_chunks_processed := 0

# 🧭 Boot Sequence
func _ready() -> void:
	engine_timer_label.text = "✅ Label is alive"
	clock_timer.timeout.connect(_on_clock_timer_timeout)
	engine_timer.timeout.connect(_on_engine_timer_timeout)
	engine_timer.wait_time = 1.0
	engine_timer.one_shot = false
	engine_timer.autostart = false

	_setup_ui()
	_connect_signals()
	
	# Initial camera setup: ensure the control panel camera is active at startup
	if cameras and cameras.has_method("switch_to_camera"):
		# Set camera 1 as the default view (Control Panel view)
		cameras.switch_to_camera(1) 

	# FIX: Configure GDExtension dependencies immediately in _ready()
	# This ensures the engine nodes are set before build_map/generate_chunk is called.
	if aetherion_engine:
		aetherion_engine.set_signals_node(aetherion_signals)
		aetherion_engine.set_tilemap(expansive_tilemap)

	if expansive_tilemap and expansive_tilemap is TileMap:
		var tileset: TileSet = expansive_tilemap.get_tileset()
		if tileset:
			tile_size = tileset.get_tile_size()

# 🧩 UI Setup
func _setup_ui() -> void:
	grid_width.max_value = 1_000_000_000
	grid_height.max_value = 1_000_000_000
	grid_width.step = 1
	grid_height.step = 1
	grid_width.value = CHUNK_SIZE_X * 2
	grid_height.value = CHUNK_SIZE_Y * 2

	placement_mode_selector.clear()
	placement_mode_selector.add_item("perlin_basic_2d")
	placement_mode_selector.add_item("checkerboard")
	placement_mode_selector.add_item("random_walk")
	placement_mode_selector.select(0)

	tile_type_selector.clear()
	tile_type_selector.add_item("Basic")
	tile_type_selector.add_item("Automata")
	tile_type_selector.select(0)

	progress_bar.min_value = 0
	progress_bar.max_value = 100
	progress_bar.value = 0
	progress_bar.visible = false

	status_label.text = "🟢 Ready to generate."

# 🔗 Signal Wiring
func _connect_signals() -> void:
	generate_button.pressed.connect(_on_generate_pressed)
	toggle_terminal_button.pressed.connect(_on_toggle_terminal_button_pressed)

	if aetherion_engine:
		if not aetherion_engine.status_updated.is_connected(_on_engine_status_updated):
			aetherion_engine.status_updated.connect(_on_engine_status_updated)

	if aetherion_signals:
		if not aetherion_signals.build_map_start.is_connected(_on_build_map_start):
			aetherion_signals.build_map_start.connect(_on_build_map_start)
		
		if not aetherion_signals.chunk_generated.is_connected(_on_chunk_generated):
			aetherion_signals.chunk_generated.connect(_on_chunk_generated)
			
		if not aetherion_signals.build_map_complete.is_connected(_on_build_map_complete):
			aetherion_signals.build_map_complete.connect(_on_build_map_complete)


# 🔄 Engine Status Poll
func _process(_delta: float) -> void:
	if aetherion_engine and aetherion_engine.has_method("get_status"):
		var status: String = aetherion_engine.call("get_status")
		status_label.text = "🧠 Engine Status: %s" % status

# 🚀 Generation Trigger
func _on_generate_pressed() -> void:
	# Guards
	if not aetherion_engine:
		push_error("AetherionEngine not found.")
		return
	if not aetherion_signals:
		push_error("AetherionSignals not found.")
		return
	if not (expansive_tilemap and expansive_tilemap is TileMap):
		push_error("expansive_tilemap is not a valid TileMap.")
		return

	var width := int(grid_width.value)
	var height := int(grid_height.value)
	var seed_text := seed_input.text
	var generator_name := placement_mode_selector.get_item_text(placement_mode_selector.selected)

	if width <= 0 or height <= 0 or width * height > 1_000_000_000:
		status_label.text = "⚠️ Invalid grid size."
		return

	var seed := int(seed_text) if seed_text.is_valid_int() else randi() % 1_000_000
	if not seed_text.is_valid_int():
		seed_input.text = str(seed)
		status_label.text = "⚠️ Invalid seed. Using random seed: %d" % seed

	# Configure dependencies and reset state
	
	progress_bar.max_value = width * height
	progress_bar.value = 0
	progress_bar.visible = true
	last_percent = -1
	total_chunks_processed = 0

	status_label.text = "🧬 Generating map with mode: %s..." % [generator_name]
	generate_button.disabled = true
	engine_timer.start()

	expansive_tilemap.clear()

	# --- CAMERA LOGIC ADDED/UPDATED ---
	# Switch to camera 2 (Map View) right before calling build_map
	if cameras and cameras.has_method("switch_to_camera"):
		cameras.switch_to_camera(2) 
		# Ensure camera 2 has a default zoom before the map calculation finishes
		if camera2:
			camera2.zoom = Vector2(1.0, 1.0)
	# ------------------------------------

	aetherion_engine.build_map(
		width, height, str(seed), generator_name
	)
	# This call triggers the Rust code's ensure_conductor() for lazy initialization (the log you see).
	print("🧪 ControlPanel: build_map called with seed %d" % seed)

# 📡 Signal Handlers

func _on_engine_status_updated(status_message: String) -> void:
	if status_message.begins_with("GENERATING:") or status_message.begins_with("ERROR:") or status_message.begins_with("IDLE:"):
		status_label.text = status_message

func _on_build_map_start() -> void:
	print("📡 Signal: build_map_start")

func _on_chunk_generated(chunk_x: int, chunk_y: int) -> void:
	# Defer the data retrieval and tile-laying to the next idle frame.
	call_deferred("_process_chunk_data", chunk_x, chunk_y)
	
	status_label.text = "🏗️ Chunk (%d, %d) signaled..." % [chunk_x, chunk_y]

# --- DEFERRED CHUNK PROCESSING WITH VALIDATION ---
func _process_chunk_data(chunk_x: int, chunk_y: int) -> void:
	# This call is safe because aetherion_engine.generate_chunk() now calls ensure_conductor()
	var chunk_dict: Dictionary = aetherion_engine.generate_chunk(chunk_x, chunk_y, 0)
	var tile_array: Array = chunk_dict.get("tiles", [])

	if tile_array.is_empty():
		push_warning("Received empty tile array for chunk (%d, %d)." % [chunk_x, chunk_y])
		return

	# --- 1. TileMap Setup Check ---
	var layer := 0
	var source_id := 0
	var tile_index := 0
	
	var tileset: TileSet = expansive_tilemap.get_tileset()
	var atlas_source: TileSetAtlasSource = null
	
	if tileset:
		atlas_source = tileset.get_source(source_id)
		if atlas_source == null:
			push_error("TileMap source ID %d not found in TileSet. Check TileSet configuration." % source_id)
			return
	else:
		push_error("TileMap is missing a TileSet. Cannot draw tiles.")
		return

	# --- 2. Iterate, Validate, and Place Tiles ---
	for tile_data_variant in tile_array:
		# FIX: Rely on implicit casting from Variant to Dictionary.
		var tile_data: Dictionary = tile_data_variant
		var tile_id: int = tile_data.get("id", 0)
		
		var local_x = tile_index % CHUNK_SIZE_X
		var local_y = tile_index / CHUNK_SIZE_X
		
		var global_x = (chunk_x * CHUNK_SIZE_X) + local_x
		var global_y = (chunk_y * CHUNK_SIZE_Y) + local_y
		
		var tile_coords = Vector2i(global_x, global_y)
		var atlas_coords = Vector2i(tile_id, 0)
		
		# Validate that the atlas coordinates point to a valid tile definition
		if atlas_source.has_tile(atlas_coords):
			# Place the tile (layer, coordinates, source, atlas_coords)
			expansive_tilemap.set_cell(layer, tile_coords, source_id, atlas_coords)
		else:
			# If invalid, erase the cell to prevent the rendering error.
			push_warning("Invalid Tile ID %d received for chunk (%d, %d). TileSet check failed. Erasing cell at (%d, %d)." % [
				tile_id, chunk_x, chunk_y, global_x, global_y
			])
			expansive_tilemap.erase_cell(layer, tile_coords)

		tile_index += 1

	# --- 3. Update Progress Bar ---
	var tiles_processed = tile_array.size()
	progress_bar.value += tiles_processed
	total_chunks_processed += 1
	
	var percent = int(float(progress_bar.value) / float(progress_bar.max_value) * 100.0)

	# Throttle visual updates
	if percent % 5 == 0 or percent == 100:
		status_label.text = "🏗️ Chunk (%d, %d) placed... %d%%" % [chunk_x, chunk_y, percent]
		expansive_tilemap.queue_redraw()
		
	last_percent = percent


func _on_build_map_complete() -> void:
	print("📡 Signal: build_map_complete")
	progress_bar.visible = false
	generate_button.disabled = false
	engine_timer.stop()
	
	var elapsed := engine_timer.wait_time - engine_timer.time_left
	
	engine_timer_label.text = "⏱️ Final Runtime: %.2fs" % elapsed
	
	# Center and clamp camera zoom (This is for camera2, the map camera)
	var width := int(grid_width.value)
	var height := int(grid_height.value)
	
	# The variable camera_tilemap is no longer declared, use camera2 directly
	if camera2: 
		camera2.global_position = Vector2(width * tile_size.x / 2.0, height * tile_size.y / 2.0)
		var zx = 1.0 / max(width / 10.0, 1.0)
		var zy = 1.0 / max(height / 10.0, 1.0)
		camera2.zoom = Vector2(clamp(zx, 0.05, 10.0), clamp(zy, 0.05, 10.0))

	expansive_tilemap.queue_redraw()

# 🕒 Clock Update
func _on_clock_timer_timeout() -> void:
	clock_label.text = "🕒 " + Time.get_datetime_string_from_system()
	engine_tick_count += 1

# 🪄 Terminal Toggle
func _on_toggle_terminal_button_pressed() -> void:
	panel_collapsed = !panel_collapsed

	for child in get_children():
		if child != toggle_terminal_button and child is Control:
			child.visible = not panel_collapsed

	clock_label.visible = not panel_collapsed
	progress_bar.visible = not panel_collapsed
	tilemap.visible = not panel_collapsed

	# --- CAMERA LOGIC ADDED/UPDATED ---
	# Call the _toggle_camera function in the 'cameras.gd' script.
	if cameras and cameras.has_method("_toggle_camera"):
		cameras._toggle_camera()
	# ------------------------------------

# ⏱️ Engine Timer Tick
func _on_engine_timer_timeout() -> void:
	var elapsed := engine_timer.wait_time - engine_timer.time_left
	engine_timer_label.text = "⏱️ Engine Runtime: %.2fs" % elapsed
	
	if aetherion_engine and aetherion_engine.has_method("tick"):
		aetherion_engine.tick(engine_tick_count)
