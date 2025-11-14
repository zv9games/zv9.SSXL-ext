class_name control_panel_utility extends RefCounted

var controller: Control

func _init(p_controller: Control) -> void:
	# Store the reference to the main ControlPanel node
	controller = p_controller

# ------------------------------------------------------------------------------
# ⚠️ CRITICAL NODE CHECKING
# ------------------------------------------------------------------------------
func check_critical_nodes() -> bool:
	"""Checks if all core FFI and rendering nodes are present and valid."""
	var valid: bool = true
	var missing_nodes: Array[String] = []

	if not is_instance_valid(controller.ssxl_engine):
		missing_nodes.append("SSXLEngine")
		valid = false
	if not is_instance_valid(controller.ssxl_signals):
		missing_nodes.append("SSXLSignals")
		valid = false
	if not is_instance_valid(controller.expansive_tilemap):
		missing_nodes.append("expansive_tilemap")
		valid = false
	if not is_instance_valid(controller.ssxl_oracle):
		missing_nodes.append("SSXLOracle")
		valid = false

	if not valid:
		var error_msg: String = "❌ CRITICAL NODES MISSING: " + ", ".join(missing_nodes)
		push_error(error_msg)
		if is_instance_valid(controller.status_label):
			controller.status_label.text = error_msg
	
	return valid

# ------------------------------------------------------------------------------
# ⌨️ INPUT HANDLING
# ------------------------------------------------------------------------------
func handle_input_event(event: InputEvent) -> void:
	"""Handles global input events delegated from _input()."""
	if event is InputEventKey and event.is_pressed() and not event.is_echo():
		if event.keycode == KEY_SPACE:
			# Check if any input field is focused (prevent spacebar from activating buttons)
			var is_input_focused: bool = (is_instance_valid(controller.seed_input) and controller.seed_input.has_focus()) or \
										 (is_instance_valid(controller.grid_width) and controller.grid_width.has_focus()) or \
										 (is_instance_valid(controller.grid_height) and controller.grid_height.has_focus())
			
			if not is_input_focused:
				# Spacebar toggles the camera view between UI and Map
				toggle_camera_view()
				controller.get_viewport().set_input_as_handled()
				

# ------------------------------------------------------------------------------
# 🔄 REDRAW THROTTLING
# ------------------------------------------------------------------------------
func request_redraw() -> void:
	"""
	Starts the redraw throttle timer if not already running.
	Limits TileMap updates to the timer's wait_time (e.g., 10 FPS).
	"""
	if not controller.redraw_pending and is_instance_valid(controller.redraw_throttle_timer):
		controller.redraw_throttle_timer.start()
		controller.redraw_pending = true

func _on_redraw_throttle_timeout() -> void:
	"""Forces a TileMap update when the throttle timer times out."""
	if controller.is_generating and is_instance_valid(controller.expansive_tilemap):
		controller.expansive_tilemap.force_update()
	controller.redraw_pending = false


# ------------------------------------------------------------------------------
# 📷 CAMERA / UI TOGGLES (Delegated Logic)
# ------------------------------------------------------------------------------

func toggle_camera_view() -> void:
	"""The handler connected to the 'Animate/Toggle View' button and SPACEBAR."""
	# Toggle the current ID: 1 becomes 2, 2 becomes 1
	var new_camera_id: int = 2 if controller.current_camera_id == 1 else 1
	switch_to_camera_view(new_camera_id)

func switch_to_camera_view(target_id: int) -> void:
	"""
	Sets the camera to the target_id (1=UI, 2=Map View) and updates the controller state.
	This is called by both toggle_camera_view and gen_logic.
	"""
	if not is_instance_valid(controller.cameras) or not controller.cameras.has_method("switch_to_camera"):
		return
		
	controller.current_camera_id = target_id
	controller.cameras.switch_to_camera(controller.current_camera_id)
	print("Camera switched to: %d" % controller.current_camera_id)

func set_map_zoom_and_position() -> void:
	"""Positions the map camera (camera2) to center the generated map and calculates appropriate zoom."""
	if not is_instance_valid(controller.camera2) or not is_instance_valid(controller.grid_width) or not is_instance_valid(controller.grid_height):
		return
		
	var map_width_tiles: int = int(controller.grid_width.value)
	var map_height_tiles: int = int(controller.grid_height.value)
	
	var full_map_width: float = float(map_width_tiles) * controller.tile_size.x
	var full_map_height: float = float(map_height_tiles) * controller.tile_size.y

	# Center the camera on the map
	controller.camera2.global_position = Vector2(full_map_width / 2.0, full_map_height / 2.0)
	
	# Calculate zoom
	var viewport_size: Vector2 = controller.get_viewport_rect().size
	var zoom_factor: float = min(viewport_size.x / full_map_width, viewport_size.y / full_map_height) * 0.9
	
	controller.camera2.zoom = Vector2(clampf(zoom_factor, 0.05, 1.0), clampf(zoom_factor, 0.05, 1.0))
	controller.initial_zoom_set = true
	print("Camera 2 positioned and zoomed after generation.")


func on_toggle_terminal_button_pressed() -> void:
	"""Toggles the visibility and size of the control panel."""
	controller.panel_collapsed = !controller.panel_collapsed
	
	var target_y: float = 32.0 if controller.panel_collapsed else 300.0
	var button_text: String = "EXPAND 🔽" if controller.panel_collapsed else "COLLAPSE 🔼"
	
	# Apply changes
	controller.custom_minimum_size.y = target_y
	if is_instance_valid(controller.toggle_terminal_button):
		controller.toggle_terminal_button.text = button_text
	
	# Hide/Show all main controls when collapsed
	for child in controller.get_children():
		if child != controller.toggle_terminal_button and child is Control:
			child.visible = not controller.panel_collapsed


# ------------------------------------------------------------------------------
# 🧹 STATE MANAGEMENT & MISC UTILITIES
# ------------------------------------------------------------------------------

func on_clock_timer_timeout() -> void:
	"""Updates the clock label."""
	if is_instance_valid(controller.clock_label):
		controller.clock_label.text = "🕒 " + Time.get_datetime_string_from_system()

func reset_temporary_state() -> void:
	"""Resets generation-specific state flags and stops timers."""
	controller.is_generating = false
	controller.redraw_pending = false
	
	if is_instance_valid(controller.generate_button):
		controller.generate_button.disabled = false
		
	if is_instance_valid(controller.engine_timer):
		controller.engine_timer.stop()
		
	if is_instance_valid(controller.animation_timer):
		controller.animation_timer.stop()
		
	controller.engine_tick_count = 0
	
func clear_generation_state() -> void:
	"""Resets all generation-related counters and labels to zero/default."""
	reset_temporary_state() # Calls the basic reset
	
	controller.total_tiles_placed = 0
	controller.generation_start_time_ms = 0
	controller.initial_zoom_set = false
	
	if is_instance_valid(controller.tiles_placed_label):
		controller.tiles_placed_label.text = "Tiles Placed: 0"
		
	if is_instance_valid(controller.tile_placement_time_label):
		controller.tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"
