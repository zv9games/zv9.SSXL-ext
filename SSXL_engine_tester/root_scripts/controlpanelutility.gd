class_name control_panel_utility extends RefCounted

# Reference to the main control_panel.gd node (the Orchestrator).
# This provides access to all UI elements, state, and external nodes.
var controller: Control

# ==============================================================================
# 1. LIFECYCLE: Constructor
# ==============================================================================
func _init(p_controller: Control) -> void:
	# Assign the reference to the main controller node.
	if is_instance_valid(p_controller):
		controller = p_controller
	else:
		push_error("❌ FATAL: ControlPanel reference missing in control_panel_utility constructor.")


# ==============================================================================
# 2. CORE VALIDATION
# ==============================================================================
# Checks for the existence and validity of all essential FFI and scene nodes.
func check_critical_nodes() -> bool:
	var valid: bool = true
	var missing_nodes: Array[String] = []

	# Check FFI GDExtension nodes
	if not is_instance_valid(controller.ssxl_engine):
		missing_nodes.append("SSXLEngine")
		valid = false
	if not is_instance_valid(controller.ssxl_signals):
		missing_nodes.append("SSXLSignals")
		valid = false
	if not is_instance_valid(controller.ssxl_oracle):
		missing_nodes.append("SSXLOracle")
		valid = false
		
	# Check Scene/Presenter nodes
	if not is_instance_valid(controller.expansive_tilemap):
		missing_nodes.append("expansive_tilemap")
		valid = false

	if not valid:
		var error_msg: String = "❌ CRITICAL NODES MISSING: " + ", ".join(missing_nodes)
		push_error(error_msg)
		if is_instance_valid(controller.status_label):
			controller.status_label.text = error_msg
	
	return valid


# ==============================================================================
# 3. FFI AND TIMER HANDLERS
# ==============================================================================

# FIX: This function was missing and is connected to the engine_timer.
# It acts as the heartbeat (polling loop) for the FFI Oracle.
func _on_engine_timer_timeout() -> void:
	print("⚙️ FFI TICKER: Polling core. Tick Count: %d" % controller.engine_tick_count)
	controller.ssxl_oracle.tick()
		
	if is_instance_valid(controller.ssxl_oracle):
		# Calling tick() tells the Rust core to process pending tasks/signals.
		controller.ssxl_oracle.tick()

# Handles the redraw_throttle_timer timeout, forcing a visual update.
func _on_redraw_throttle_timeout() -> void:
	if controller.redraw_pending and is_instance_valid(controller.expansive_tilemap):
		# force_update() ensures all tiles placed in the background are drawn in one batch.
		controller.expansive_tilemap.force_update()
		controller.redraw_pending = false

# Triggers a redraw cycle by starting the throttle timer.
func request_redraw() -> void:
	controller.redraw_pending = true
	if is_instance_valid(controller.redraw_throttle_timer) and not controller.redraw_throttle_timer.is_stopped():
		controller.redraw_throttle_timer.start()

# Updates the clock label with the current system time.
func on_clock_timer_timeout() -> void:
	if is_instance_valid(controller.clock_label):
		var time_string: String = Time.get_datetime_string_from_system()
		controller.clock_label.text = "🕒 " + time_string


# ==============================================================================
# 4. INPUT AND UI HANDLERS
# ==============================================================================
# Handles global input events for hotkeys (F5, F6, F7).
func handle_input_event(event: InputEvent) -> void:
	if event is InputEventKey and event.is_pressed() and not event.is_echo():
		
		# F5: Start Generation (Delegates to gen_logic sub-script)
		if event.keycode == KEY_F5:
			# Safety check: Only run if generation is not already active.
			if not controller.is_generating:
				controller.gen_logic.start_generation()
			controller.get_viewport().set_input_as_handled()
			return
		
		# F6: Toggle Camera View
		elif event.keycode == KEY_F6:
			toggle_camera_view()
			controller.get_viewport().set_input_as_handled()
			return
		
		# F7: Toggle Control Panel Visibility
		elif event.keycode == KEY_F7:
			on_toggle_terminal_button_pressed()
			controller.get_viewport().set_input_as_handled()
			return

# Toggles visibility of all control panel elements except the status billboard and button.
func on_toggle_terminal_button_pressed() -> void:
	controller.panel_collapsed = not controller.panel_collapsed
	
	for child in controller.get_children():
		# Skip the control elements that must *always* be visible
		if child == controller.toggle_terminal_button or child == controller.status_label:
			continue
			
		# Toggle visibility for all other UI controls
		if child is Control:
			child.visible = not controller.panel_collapsed

# Toggles the active camera view between camera1 and camera2.
func toggle_camera_view() -> void:
	var cameras_node: Node = controller.main.get_node("cameras")
	
	if is_instance_valid(cameras_node) and cameras_node.has_method("_toggle_camera"):
		# Delegate the camera switch to the dedicated Cameras node manager.
		cameras_node._toggle_camera()
		
		# Update the controller's state variable based on a simple toggle between 1 and 2.
		controller.current_camera_id = 1 if controller.current_camera_id == 2 else 2


# ==============================================================================
# 5. SYSTEM RESET
# ==============================================================================
# Resets all state variables and timers after a map generation process completes.
func reset_system_state() -> void:
	controller.is_generating = false
	controller.redraw_pending = false
	controller.engine_tick_count = 0
	
	# Stop all timers
	if is_instance_valid(controller.engine_timer):
		controller.engine_timer.stop()
	if is_instance_valid(controller.animation_timer):
		controller.animation_timer.stop()
	if is_instance_valid(controller.redraw_throttle_timer):
		controller.redraw_throttle_timer.stop()
		
	# Reset state metrics
	controller.total_tiles_placed = 0
	controller.generation_start_time_ms = 0
	controller.initial_zoom_set = false
	
	# Reset UI elements
	if is_instance_valid(controller.generate_button):
		controller.generate_button.disabled = false
		
	if is_instance_valid(controller.tiles_placed_label):
		controller.tiles_placed_label.text = "Tiles Placed: 0"
		
	if is_instance_valid(controller.progress_bar):
		controller.progress_bar.value = 0.0
		controller.progress_bar.max_value = 0.0
		controller.progress_bar.visible = false


# ==============================================================================
# 6. GENERATION CONTROL (FORWARDED TO SSXLEngine)
# ==============================================================================

func start_generation() -> void:
	if controller.is_generating:
		return
	controller.is_generating = true
	controller.ssxl_engine.build_map(
		controller.map_width,
		controller.map_height,
		controller.seed_input.text,
		controller.generator_selector.get_item_text(controller.generator_selector.selected)
	)

func stop_generation() -> void:
	if not controller.is_generating:
		return
	print("GEN_LOGIC: Stop command issued via FFI")
	controller.ssxl_engine.stop_generation()   # ← THIS IS THE CALL YOU WERE MISSING
	controller.is_generating = false
