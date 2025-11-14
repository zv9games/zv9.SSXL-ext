class_name control_panel_gen_logic extends RefCounted

var controller: Control

func _init(p_controller: Control) -> void:
	# Store the reference to the main ControlPanel node
	controller = p_controller

# ------------------------------------------------------------------------------
# ⏯️ TOGGLE (UI ENTRY POINT)
# ------------------------------------------------------------------------------
func toggle_generation() -> void:
	"""
	Toggles map generation between start and stop based on the current state.
	This is the function connected directly to the 'Generate' button.
	"""
	if controller.is_generating:
		stop_generation()
	else:
		start_generation()

# ------------------------------------------------------------------------------
# 🛑 STOP GENERATION LOGIC
# ------------------------------------------------------------------------------
func stop_generation() -> void:
	print("🛑 ControlPanel: Stop button pressed. Halting generation.")
	
	# Attempt to send a stop command to the FFI/Rust engine
	if is_instance_valid(controller.ssxl_engine) and controller.ssxl_engine.has_method("send_stop_command"): # Refactored method name for clarity
		controller.ssxl_engine.send_stop_command()
		print("🛑 ControlPanel: Sent explicit stop command to SSXL Engine.")
		
	# Reset state and update UI
	controller.utility.reset_temporary_state()
	
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "🛑 Generation halted by user."
	if is_instance_valid(controller.progress_bar):
		controller.progress_bar.visible = false
	if is_instance_valid(controller.generate_button):
		controller.generate_button.text = "IGNITION"
	
	# Switch back to the Control Panel camera
	controller.utility.switch_to_camera_view(1) # Delegated to utility script
	print("➡️ Switched to Camera 1 (Control Panel View).")


# ------------------------------------------------------------------------------
# 🟢 START GENERATION LOGIC
# ------------------------------------------------------------------------------
func start_generation() -> void:
	# Critical node checks
	if not controller.utility.check_critical_nodes():
		return # Error message handled in utility

	# Input gathering: Grid size
	var width: int = int(controller.grid_width.value) if is_instance_valid(controller.grid_width) else 0
	var height: int = int(controller.grid_height.value) if is_instance_valid(controller.grid_height) else 0
	
	# Input validation
	if width <= 0 or height <= 0:
		if is_instance_valid(controller.status_label):
			controller.status_label.text = "⚠️ Invalid grid size."
		return

	# Retrieve inputs using safer, single-purpose functions
	var generator_name: String = get_selector_item_text(controller.placement_mode_selector, "perlin_basic_2d", "Generator list empty. Using default: %s")
	var tile_type_name: String = get_selector_item_text(controller.tile_type_selector, "tile_type_grass")
	var seed: int = get_and_set_seed()
		
	# State Setup
	if controller.ssxl_engine.has_method("set_generator"):
		controller.ssxl_engine.set_generator(tile_type_name)
		print("⚙️ ControlPanel: set_generator called with tile_type %s" % tile_type_name)

	controller.utility.clear_generation_state()
	controller.is_generating = true
	controller.is_animated = controller.animate_checkbox.button_pressed
	
	controller.animation_logic.setup_animation_worker(controller.is_animated)

	controller.generation_start_time_ms = Time.get_ticks_msec()
	
	# Progress Bar Setup
	var total_chunks_x: float = ceil(float(width) / controller.CHUNK_SIZE_X)
	var total_chunks_y: float = ceil(float(height) / controller.CHUNK_SIZE_Y)
	
	if is_instance_valid(controller.progress_bar):
		var total_tiles: float = total_chunks_x * total_chunks_y * controller.CHUNK_SIZE_X * controller.CHUNK_SIZE_Y
		controller.progress_bar.max_value = total_tiles
		controller.progress_bar.value = 0.0
		controller.progress_bar.visible = true
	
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "🧬 Generating map with mode: %s..." % [generator_name]
	
	if is_instance_valid(controller.generate_button):
		controller.generate_button.disabled = false
		controller.generate_button.text = "STOP"
	
	prepopulate_map(width, height)
	controller.get_tree().process_frame

	# Camera switch to map view
	controller.utility.switch_to_camera_view(2)
	print("➡️ Switched to Camera 2 (Map View).")

	# FFI CALL 2: Queues the task in the Rust engine
	controller.ssxl_engine.build_map(width, height, str(seed), generator_name)
	print("🧪 ControlPanel: build_map called with seed %d and generator %s (Task Queued)" % [seed, generator_name])

	# Start the appropriate timer based on animation state
	if controller.is_animated and is_instance_valid(controller.animation_timer):
		controller.animation_timer.start()
	elif is_instance_valid(controller.engine_timer):
		controller.engine_timer.start()

# ------------------------------------------------------------------------------
# ⚙️ UTILITY FUNCTIONS
# ------------------------------------------------------------------------------

func get_selector_item_text(selector: OptionButton, default_value: String, warning_message: String = "") -> String:
	"""Safely retrieves the selected item text from an OptionButton, or returns a default."""
	if not is_instance_valid(selector) or selector.get_item_count() == 0:
		if not warning_message.is_empty() and is_instance_valid(controller.status_label):
			controller.status_label.text = warning_message % default_value
		return default_value
		
	var selected_idx: int = max(0, selector.selected)
	if selected_idx < selector.get_item_count():
		return selector.get_item_text(selected_idx)
		
	return default_value # Should not happen if item_count > 0, but safe fallback

func get_and_set_seed() -> int:
	"""Retrieves the seed from input or generates a new random one, updating the UI."""
	if is_instance_valid(controller.seed_input) and controller.seed_input.text.is_valid_int():
		return int(controller.seed_input.text)
	
	var new_seed: int = randi() % 1_000_000
	if is_instance_valid(controller.seed_input):
		controller.seed_input.text = str(new_seed)
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "⚠️ Invalid seed. Using random seed: %d" % new_seed
	
	return new_seed

func prepopulate_map(width: int, height: int) -> void:
	"""Pre-fills the entire map with a base tile to reserve space and show bounds."""
	if not is_instance_valid(controller.expansive_tilemap):
		push_error("Cannot prepopulate: TileMap invalid.")
		return

	controller.expansive_tilemap.clear_layer(0)
	for x in range(width):
		for y in range(height):
			controller.expansive_tilemap.set_cell(0, Vector2i(x, y), controller.BASE_SOURCE_ID, controller.BASE_ATLAS_COORDS, controller.BASE_ALT_ID)
	
	# Note: force_update() is called after build_map_complete for final drawing.
	print("🎨 ControlPanel: Map prepopulated with base tile (%d x %d)." % [width, height])


# ------------------------------------------------------------------------------
# ✅ COMPLETION HANDLER
# ------------------------------------------------------------------------------
func _on_build_map_complete_received() -> void:
	# This function is connected directly to the ssxl_signals build_map_complete signal
	on_build_map_complete()
	
func on_build_map_complete() -> void:
	print("✅ Finalizing map generation.")
	
	if is_instance_valid(controller.progress_bar):
		controller.progress_bar.visible = false
	
	var elapsed_placement_time_ms: int = Time.get_ticks_msec() - controller.generation_start_time_ms
	var elapsed_placement_time_sec: float = float(elapsed_placement_time_ms) / 1000.0
	
	var final_tile_count: int = controller.total_tiles_placed
	
	# Update labels
	if is_instance_valid(controller.tiles_placed_label):
		controller.tiles_placed_label.text = "Tiles Placed: %d" % final_tile_count
		
	if is_instance_valid(controller.tile_placement_time_label):
		controller.tile_placement_time_label.text = "⏱️ Tile Placement Time: %.2fs" % elapsed_placement_time_sec
		
	if is_instance_valid(controller.engine_timer_label):
		controller.engine_timer_label.text = "⏱️ Final Runtime: %.2fs | Ticks: %d" % [elapsed_placement_time_sec, controller.engine_tick_count]

	# Final camera positioning and zoom (Delegate this logic to Utility)
	controller.utility.set_map_zoom_and_position()
	
	if is_instance_valid(controller.expansive_tilemap):
		controller.expansive_tilemap.force_update()
	
	controller.get_tree().process_frame

	controller.utility.reset_temporary_state()
	
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "✅ Generation Complete. Map built. (%d tiles)" % final_tile_count
	if is_instance_valid(controller.generate_button):
		controller.generate_button.text = "IGNITION"
