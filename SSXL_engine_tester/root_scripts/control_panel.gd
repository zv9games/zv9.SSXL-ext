extends Control

# ------------------------------------------------------------------------------
# --- CHUNK & RENDER CONSTANTS ---
# ------------------------------------------------------------------------------
const CHUNK_SIZE_X: int = 32 # Confirmed 32 to match Rust core (32x32)
const CHUNK_SIZE_Y: int = 32 # Confirmed 32 to match Rust core (32x32)
const DEFAULT_CHUNKS: int = 12

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
@onready var animation_timer: Timer = $AnimationTimer # NEW: Reference to the slow animation timer
@onready var engine_timer_label: Label = $enginetimerlabel
@onready var tiles_placed_label: Label = $tilesplacedlabel
@onready var tile_placement_time_label: Label = $tiletimeofplacement
@onready var redraw_throttle_timer: Timer = $redrawthrottle

# 🧠 External Scene References - Use explicit paths for non-siblings
@onready var main: Node2D = get_parent() as Node2D
@onready var ssxl_engine: Node = main.get_node("SSXLEngine")
@onready var ssxl_oracle: Node = main.get_node("SSXLOracle")
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
var generation_start_time_ms: int = 0
var redraw_pending: bool = false
var is_animated: bool = false # NEW: Animation State
var animation_tilemap_handle: TileMap # NEW: Store TileMap handle for main-thread animation updates


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

	# Initial setting of animation state
	_on_animate_checkbox_toggled(animate_checkbox.button_pressed)


func _setup_timers() -> void:
	# FAST TIMER (for non-animated mode)
	engine_timer.wait_time = 0.01 # 100 Hz update rate (FAST)
	engine_timer.one_shot = false
	engine_timer.autostart = false
	engine_timer.timeout.connect(_on_engine_timer_timeout)
	
	# SLOW ANIMATION TIMER (for animated mode)
	if animation_timer:
		animation_timer.wait_time = 0.5 # 2 Hz update rate (SLOW/ANIMATED)
		animation_timer.one_shot = false
		animation_timer.autostart = false
		animation_timer.timeout.connect(_on_animation_timer_timeout)
	
	if clock_timer:
		clock_timer.timeout.connect(_on_clock_timer_timeout)
		
	if redraw_throttle_timer:
		redraw_throttle_timer.one_shot = true
		redraw_throttle_timer.wait_time = 0.1

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
	
	# Set default grid to 384 x 384 (12 chunks * 32)
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
	
	if is_instance_valid(tile_placement_time_label):
		tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"

	status_label.text = "🟢 Ready to generate."
	generate_button.text = "IGNITION"


func _connect_signals() -> void:
	generate_button.pressed.connect(_on_generate_pressed)
	toggle_terminal_button.pressed.connect(_on_toggle_terminal_button_pressed)
	animate_checkbox.toggled.connect(_on_animate_checkbox_toggled) # NEW: Animation toggle signal
	
	if redraw_throttle_timer and not redraw_throttle_timer.timeout.is_connected(_on_redraw_throttle_timeout):
		redraw_throttle_timer.timeout.connect(_on_redraw_throttle_timeout)

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
# 🎮 INPUT HANDLING
# ------------------------------------------------------------------------------
func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.is_pressed() and not event.is_echo():
		if event.keycode == KEY_SPACE:
			var focused_control: Control = get_viewport().gui_get_focus_owner()
			
			if focused_control is LineEdit or focused_control is SpinBox:
				return

			if is_generating or not generate_button.disabled:
				_toggle_camera()
				get_viewport().set_input_as_handled()
				
# ------------------------------------------------------------------------------
# 🔄 ENGINE LOOP (SIMPLIFIED)
# ------------------------------------------------------------------------------
func _process(_delta: float) -> void:
	if not is_generating:
		return
		
	# Update Engine Status Label
	if ssxl_engine and ssxl_engine.has_method("get_status"):
		var status: String = ssxl_engine.call("get_status")
		status_label.text = "🧠 Engine Status: %s" % status


# ------------------------------------------------------------------------------
# 🚀 GENERATION START & END
# ------------------------------------------------------------------------------
func _on_generate_pressed() -> void:
	# --- STOP GENERATION LOGIC (Toggle functionality) ---
	if is_generating:
		print("🛑 ControlPanel: Stop button pressed. Halting generation.")
		_reset_temporary_state()
		status_label.text = "🛑 Generation halted by user."
		progress_bar.visible = false
		generate_button.text = "IGNITION"
		
		# Switch camera back to UI view
		if cameras and cameras.has_method("switch_to_camera"):
			current_camera_id = 1
			cameras.switch_to_camera(current_camera_id)
		return
	# -----------------------------------------------------
		
	# --- START GENERATION LOGIC ---
		
	if not ssxl_engine or not ssxl_signals or not is_instance_valid(expansive_tilemap) or not ssxl_oracle:
		push_error("Critical external node missing. Cannot start generation.")
		return

	# Input gathering
	var width: int = int(grid_width.value)
	var height: int = int(grid_height.value)
	var generator_name: String = placement_mode_selector.get_item_text(placement_mode_selector.selected)
	var tile_type_name: String = tile_type_selector.get_item_text(tile_type_selector.selected)

	# Seed handling
	var seed: int
	if seed_input.text.is_valid_int():
		seed = int(seed_input.text)
	else:
		seed = randi() % 1_000_000
		seed_input.text = str(seed)
		status_label.text = "⚠️ Invalid seed. Using random seed: %d" % seed
		
	# --- CHUNK LIMIT UPDATE ---
	if width <= 0 or height <= 0 or (float(width) / CHUNK_SIZE_X) * (float(height) / CHUNK_SIZE_Y) > 50_000.0:
		status_label.text = "⚠️ Invalid grid size. Max chunks limit exceeded."
		return

	# --- DEFENSIVE RE-LINK AND STATE RESET ---
	if ssxl_engine.has_method("set_signals_node"):
		ssxl_engine.set_signals_node(ssxl_signals)
	if ssxl_engine.has_method("set_tilemap"):
		ssxl_engine.set_tilemap(expansive_tilemap)
		
	# Reset Oracle/Engine state
	if ssxl_oracle.has_method("reset"):
		ssxl_oracle.reset()
	# --- END DEFENSIVE SETUP ---

	# Set the tile configuration/generator type before calling build_map
	if ssxl_engine.has_method("set_generator"):
		ssxl_engine.set_generator(tile_type_name)
		print("⚙️ ControlPanel: set_generator called with tile_type %s" % tile_type_name)


	# State Setup
	_clear_generation_state()
	is_generating = true
	initial_zoom_set = false
	is_animated = animate_checkbox.button_pressed # Get the current state
	
	# Record the exact start time before the FFI call
	generation_start_time_ms = Time.get_ticks_msec()
	
	# Max value calculation (Total Tiles in the map)
	var total_chunks_x: float = ceil(float(width) / CHUNK_SIZE_X)
	var total_chunks_y: float = ceil(float(height) / CHUNK_SIZE_Y)
	progress_bar.max_value = float(total_chunks_x * total_chunks_y * CHUNK_SIZE_X * CHUNK_SIZE_Y)
	progress_bar.value = 0.0
	progress_bar.visible = true
	
	status_label.text = "🧬 Generating map with mode: %s..." % [generator_name]
	generate_button.disabled = false # Keep enabled so the user can press 'STOP'
	generate_button.text = "STOP" # Set button text to STOP
	
	# Clear map
	expansive_tilemap.clear()
	expansive_tilemap.emit_signal("tile_data_changed", 0, Vector2i.ZERO, Vector2i(width, height))
	
	await get_tree().process_frame

	# Camera switch to map view
	if cameras and cameras.has_method("switch_to_camera"):
		current_camera_id = 2
		cameras.switch_to_camera(current_camera_id)

	# FFI CALL: Animation Mode Setup (New method call)
	# 🛑 REMOVED: ssxl_engine.set_animation_mode(is_animated)
	if is_animated:
		if ssxl_engine.has_method("start_loading_animation"):
			ssxl_engine.start_loading_animation(2.0) # Set desired FPS for animation worker
			print("⚙️ ControlPanel: start_loading_animation(2.0) called.")
	

	# FFI CALL 2: Queues the task in the Rust engine
	ssxl_engine.build_map(width, height, str(seed), generator_name)
	print("🧪 ControlPanel: build_map called with seed %d and generator %s (Task Queued)" % [seed, generator_name])

	# CRITICAL FIX: Start the appropriate timer based on animation state
	if is_animated and animation_timer:
		animation_timer.start()
		print("🚀 Animation Timer started (0.5s/tick). SSXLOracle::tick() will now drive generation.")
	else:
		engine_timer.start()
		print("🚀 Engine Timer started (0.01s/tick). SSXLOracle::tick() will now drive generation.")


# --- COMPLETION HANDLER ---
func _on_build_map_complete_received() -> void:
	print("📡 Signal: build_map_complete received. Processing final state.")
	_on_build_map_complete()
	
func _on_build_map_complete() -> void:
	print("✅ Finalizing map generation.")
	progress_bar.visible = false
	
	var elapsed_placement_time_ms: int = Time.get_ticks_msec() - generation_start_time_ms
	var elapsed_placement_time_sec: float = float(elapsed_placement_time_ms) / 1000.0
	
	var final_tile_count: int = total_tiles_placed
	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: %d" % final_tile_count
		
	if is_instance_valid(tile_placement_time_label):
		tile_placement_time_label.text = "⏱️ Tile Placement Time: %.2fs" % elapsed_placement_time_sec
		
	engine_timer_label.text = "⏱️ Final Runtime: %.2fs | Ticks: %d" % [elapsed_placement_time_sec, engine_tick_count]

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

	expansive_tilemap.force_update()
	
	await get_tree().process_frame

	_reset_temporary_state()
	status_label.text = "✅ Generation Complete. Map built. (%d tiles)" % final_tile_count
	generate_button.text = "IGNITION"


# ------------------------------------------------------------------------------
# 📡 SIGNAL HANDLERS
# ------------------------------------------------------------------------------
func _on_engine_status_updated(status_message: String) -> void:
	if status_message.begins_with("GENERATING:") or status_message.begins_with("ERROR:") or status_message.begins_with("IDLE:"):
		status_label.text = status_message

func _on_build_map_start() -> void:
	print("📡 Signal: build_map_start")

func _on_chunk_generated(chunk_x: int, chunk_y: int) -> void:
	# In the new model, the Rust engine has already called set_cell on the TileMap.
	
	var tiles_in_chunk: int = CHUNK_SIZE_X * CHUNK_SIZE_Y
	
	# 1. Update Trackers
	progress_bar.value += float(tiles_in_chunk)
	total_tiles_placed += tiles_in_chunk
	
	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: %d" % total_tiles_placed
	
	# 2. Request Throttled Redraw
	_request_redraw()

	var percent: int = int(progress_bar.value / progress_bar.max_value * 100.0)

	# 3. THROTTLED STATUS UPDATE
	if percent != last_percent and (percent % 1 == 0 or percent == 100):
		status_label.text = "🏗️ Chunk (%d, %d) applied... %d%%" % [chunk_x, chunk_y, percent]
		
	last_percent = percent
	
func _on_generation_error(error_message: String) -> void:
	print("❌ ERROR: Generation failed: %s" % error_message)
	_reset_temporary_state()
	status_label.text = "❌ ERROR: Generation failed. Check console."
	generate_button.text = "IGNITION"

# ------------------------------------------------------------------------------
# ⚙️ ANIMATION CONTROL
# ------------------------------------------------------------------------------
func _on_animate_checkbox_toggled(button_pressed: bool) -> void:
	is_animated = button_pressed
	print("⚙️ Animation Toggled: %s" % is_animated)

	if not ssxl_engine or not ssxl_engine.has_method("send_animation_command"):
		push_error("SSXLEngine or send_animation_command method not available.")
		return
	
	# 1. Timer Management (Set fast/slow Oracle tick)
	if is_animated:
		engine_timer.stop()
		animation_timer.start()
		
		# 2. FFI Call - Set FPS
		# 🛑 REMOVED: ssxl_engine.set_animation_mode(is_animated)
		if ssxl_engine.has_method("start_loading_animation"):
			ssxl_engine.start_loading_animation(2.0) # Set desired FPS
			print("⚙️ ControlPanel: start_loading_animation(2.0) called.")

		# 3. Send StartTestAnimation command to Rust worker (Unit variant, no argument)
		ssxl_engine.send_animation_command("StartTestAnimation")
		print("⚙️ ControlPanel: Sent StartTestAnimation command.")

		# 4. Store the TileMap handle in the ControlPanel for main-thread consumption.
		animation_tilemap_handle = expansive_tilemap
		
	else:
		animation_timer.stop()
		engine_timer.start()

		# 2. FFI Call - Stop the worker
		ssxl_engine.send_animation_command("StopTestAnimation")
		print("⚙️ ControlPanel: Sent StopTestAnimation command.")
		
		# 3. Clear the handle
		animation_tilemap_handle = null
		
	if is_generating:
		# Note: Changing animation mode mid-generation is typically disallowed/unreliable.
		status_label.text = "⚠️ Animation mode set for next generation. Restart required."

# ------------------------------------------------------------------------------
# ⏱️ TIMER TIMEOUTS
# ------------------------------------------------------------------------------

# Central function to drive the engine tick and update runtime display
func _handle_engine_tick() -> void:
	if ssxl_oracle and ssxl_oracle.has_method("tick"):
		ssxl_oracle.tick()
		engine_tick_count += 1 # Increment tick after a successful tick call

	# Update Runtime Display
	var elapsed_ms: int = Time.get_ticks_msec() - generation_start_time_ms
	var elapsed_sec: float = float(elapsed_ms) / 1000.0
	engine_timer_label.text = "⏱️ Runtime: %.2fs | Ticks: %d" % [elapsed_sec, engine_tick_count]

func _on_engine_timer_timeout() -> void:
	# This is the FAST tick (0.01s)
	_handle_engine_tick()
	
func _on_animation_timer_timeout() -> void:
	# This is the SLOW tick (0.5s)
	_handle_engine_tick()

func _on_clock_timer_timeout() -> void:
	# Only updates the system clock display
	clock_label.text = "🕒 " + Time.get_datetime_string_from_system()

# ------------------------------------------------------------------------------
# --- UTILITY AND MISC. ---
# ------------------------------------------------------------------------------
## Starts the redraw throttle timer if not already running.
func _request_redraw() -> void:
	if not redraw_pending and redraw_throttle_timer:
		redraw_throttle_timer.start()
		redraw_pending = true

## Executes the expensive TileMap redraw when the throttle timer runs out.
func _on_redraw_throttle_timeout() -> void:
	if is_generating:
		expansive_tilemap.force_update()
	redraw_pending = false

func _toggle_camera() -> void:
	if not cameras or not cameras.has_method("switch_to_camera"):
		return
		
	if current_camera_id == 1:
		current_camera_id = 2
	else:
		current_camera_id = 1
		
	cameras.switch_to_camera(current_camera_id)
	print("Camera toggled to: %d" % current_camera_id)

func _reset_temporary_state() -> void:
	is_generating = false
	generate_button.disabled = false
	engine_timer.stop()
	if animation_timer:
		animation_timer.stop() # Added stop for the animation timer
	engine_tick_count = 0

func _clear_generation_state() -> void:
	_reset_temporary_state()
	
	total_tiles_placed = 0
	generation_start_time_ms = 0
	if is_instance_valid(tiles_placed_label):
		tiles_placed_label.text = "Tiles Placed: 0"
	if is_instance_valid(tile_placement_time_label):
		tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"

func _on_toggle_terminal_button_pressed() -> void:
	panel_collapsed = !panel_collapsed

	for child in get_children():
		if child != toggle_terminal_button and child is Control and child != status_label and child != engine_timer_label and child != tiles_placed_label and child != tile_placement_time_label:
			child.visible = not panel_collapsed
	
	if clock_label:
		clock_label.visible = not panel_collapsed
	if progress_bar:
		progress_bar.visible = not panel_collapsed and is_generating
	
	if tilemap_node:
		tilemap_node.visible = not panel_collapsed
