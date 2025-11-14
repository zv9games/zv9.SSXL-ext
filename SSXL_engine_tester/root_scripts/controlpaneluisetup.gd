class_name control_panel_ui_setup extends RefCounted

var controller: Control

func _init(p_controller: Control) -> void:
	# Store the reference to the main ControlPanel node
	controller = p_controller


# ------------------------------------------------------------------------------
## 🛠️ CORE LIFECYCLE
# ------------------------------------------------------------------------------
func setup_all() -> void:
	"""Combines all setup functions for the main controller's _ready()."""
	setup_timers()
	setup_engine_links()
	setup_ui()
	connect_signals()
	print("⚙️ ControlPanel UI Setup complete.")


# ------------------------------------------------------------------------------
## ⏱️ TIMER CONFIGURATION
# ------------------------------------------------------------------------------
func setup_timers() -> void:
	"""Sets up timer properties and connects timeouts to the appropriate handler."""
	
	# Engine Timer (Fast poll for non-animated generation)
	if is_instance_valid(controller.engine_timer):
		controller.engine_timer.wait_time = 0.01
		controller.engine_timer.one_shot = false
		controller.engine_timer.autostart = false
	
	# Animation Timer (Throttled tick for animated generation)
	if is_instance_valid(controller.animation_timer):
		controller.animation_timer.wait_time = 0.5
		controller.animation_timer.one_shot = false
		controller.animation_timer.autostart = false
	
	# Clock Timer
	if is_instance_valid(controller.clock_timer):
		# Connect to the Utility module for clock updates
		controller.clock_timer.timeout.connect(controller.utility.on_clock_timer_timeout)
		
	# Redraw Throttle Timer (Map Gen batch redraw)
	if is_instance_valid(controller.redraw_throttle_timer):
		controller.redraw_throttle_timer.one_shot = true
		controller.redraw_throttle_timer.wait_time = 0.1


# ------------------------------------------------------------------------------
## 🔗 EXTERNAL NODE LINKING
# ------------------------------------------------------------------------------
func setup_engine_links() -> void:
	"""Passes necessary Godot node references to the SSXL engine components."""
	if is_instance_valid(controller.ssxl_engine):
		if is_instance_valid(controller.ssxl_signals) and controller.ssxl_engine.has_method("set_signals_node"):
			controller.ssxl_engine.set_signals_node(controller.ssxl_signals)
		if is_instance_valid(controller.expansive_tilemap) and controller.ssxl_engine.has_method("set_tilemap"):
			controller.ssxl_engine.set_tilemap(controller.expansive_tilemap)
	else:
		push_error("❌ SSXLEngine node not found.")

# ------------------------------------------------------------------------------
## 📐 UI INITIALIZATION (Populate OptionButtons)
# ------------------------------------------------------------------------------
func setup_ui() -> void:
	"""Sets initial values and properties for core UI elements and populates selectors."""
	
	# 1. Grid Size SpinBoxes
	if is_instance_valid(controller.grid_width) and is_instance_valid(controller.grid_height):
		# Set sensible bounds and default values based on CHUNK_SIZE
		controller.grid_width.max_value = 1_000_000_000.0
		controller.grid_width.step = float(controller.CHUNK_SIZE_X)
		controller.grid_width.value = float(controller.CHUNK_SIZE_X * controller.DEFAULT_CHUNKS)
		
		controller.grid_height.max_value = 1_000_000_000.0
		controller.grid_height.step = float(controller.CHUNK_SIZE_Y)
		controller.grid_height.value = float(controller.CHUNK_SIZE_Y * controller.DEFAULT_CHUNKS)

	# 2. Tile Type Selector
	if is_instance_valid(controller.tile_type_selector):
		controller.tile_type_selector.clear()
		controller.tile_type_selector.add_item("tile_type_grass")
		controller.tile_type_selector.add_item("tile_type_water")
		controller.tile_type_selector.add_item("tile_type_mountain")
		controller.tile_type_selector.select(0)
	
	# 3. Placement Mode Selector
	if is_instance_valid(controller.placement_mode_selector):
		controller.placement_mode_selector.clear()
		controller.placement_mode_selector.add_item("perlin_basic_2d")
		controller.placement_mode_selector.add_item("cellular_automata_checkerboard")
		controller.placement_mode_selector.add_item("drunkards_walk")
		controller.placement_mode_selector.add_item("maze_recursive_division")
		controller.placement_mode_selector.select(0)
		
	# 4. Status and Progress Bar
	if is_instance_valid(controller.progress_bar):
		controller.progress_bar.visible = false
	
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "IDLE: Ready for Ignition."


# ------------------------------------------------------------------------------
## 🔌 SIGNAL CONNECTIONS
# ------------------------------------------------------------------------------
func connect_signals() -> void:
	"""Connects all UI and external engine signals to the appropriate handler functions."""
	
	# --- UI Connections (Delegated to helper scripts) ---
	if is_instance_valid(controller.generate_button):
		controller.generate_button.pressed.connect(controller.gen_logic.toggle_generation)
	
	if is_instance_valid(controller.animate_ui_button):
		controller.animate_ui_button.pressed.connect(controller.utility.toggle_camera_view)
		
	if is_instance_valid(controller.toggle_terminal_button):
		controller.toggle_terminal_button.pressed.connect(controller.utility.on_toggle_terminal_button_pressed)
		
	if is_instance_valid(controller.animate_checkbox):
		controller.animate_checkbox.toggled.connect(controller.animation_logic.on_animate_checkbox_toggled)
	
	# --- External Engine Connections (SSXLSignals Hub) ---

	if is_instance_valid(controller.ssxl_signals):
		# Signals related to map start/chunk/error go to the dedicated Signal Handler
		controller.ssxl_signals.build_map_start.connect(controller.signal_handler._on_build_map_start)
		controller.ssxl_signals.chunk_generated.connect(controller.signal_handler._on_chunk_generated)
		controller.ssxl_signals.generation_error.connect(controller.signal_handler._on_generation_error)
		
		# Completion signal connects directly to the Generation Logic to finalize the process
		controller.ssxl_signals.build_map_complete.connect(controller.gen_logic._on_build_map_complete_received)
