var controller: Control

func _init(p_controller: Control) -> void:
	controller = p_controller

func _on_ffi_alignment_complete() -> void:
	_connect_local_ui_signals()
	_connect_local_timers()

func _connect_local_ui_signals() -> void:
	var gen_callable = Callable(controller.gen_logic, "start_generation")
	var animate_callable = Callable(controller.animation_logic, "_on_animate_button_pressed")
	var toggle_callable = Callable(controller.utility, "on_toggle_terminal_button_pressed")
	var width_callable = Callable(controller.ui_setup, "_on_grid_size_value_changed")
	var height_callable = Callable(controller.ui_setup, "_on_grid_size_value_changed")
	
	if controller.generate_button.pressed.is_connected(gen_callable):
		controller.generate_button.pressed.disconnect(gen_callable)
	controller.generate_button.pressed.connect(gen_callable)

	if controller.animate_ui_button.pressed.is_connected(animate_callable):
		controller.animate_ui_button.pressed.disconnect(animate_callable)
	controller.animate_ui_button.pressed.connect(animate_callable)
	
	if controller.toggle_terminal_button.pressed.is_connected(toggle_callable):
		controller.toggle_terminal_button.pressed.disconnect(toggle_callable)
	controller.toggle_terminal_button.pressed.connect(toggle_callable)
	
	if controller.grid_width.value_changed.is_connected(width_callable):
		controller.grid_width.value_changed.disconnect(width_callable)
	controller.grid_width.value_changed.connect(width_callable)
	
	if controller.grid_height.value_changed.is_connected(height_callable):
		controller.grid_height.value_changed.disconnect(height_callable)
	controller.grid_height.value_changed.connect(height_callable)

func _connect_local_timers() -> void:
	var engine_timer_callable = Callable(controller.animation_logic, "_on_engine_timer_timeout")
	if is_instance_valid(controller.engine_timer) and not controller.engine_timer.timeout.is_connected(engine_timer_callable):
		controller.engine_timer.timeout.connect(engine_timer_callable)

func _on_engine_status_updated(status_message: String) -> void:
	if not is_instance_valid(controller.status_label):
		return
	print("DEBUG: Engine status updated →", status_message)
	controller.status_label.text = status_message

func _on_generation_error(error_message: String) -> void:
	print("DEBUG: Generation error →", error_message)
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "ERROR: Generation failed. Check console."
	if is_instance_valid(controller.generate_button):
		controller.generate_button.text = "IGNITION"
	controller.utility.reset_system_state()

func _on_build_map_start() -> void:
	print("DEBUG: Build map start signal received")
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "Map Generation Initiated..."

func _on_chunk_generated(chunk_x: int, chunk_y: int, tiles_in_chunk: int) -> void:
	if not controller.is_generating:
		return
	print("DEBUG: Chunk generated → (%d, %d) with %d tiles" % [chunk_x, chunk_y, tiles_in_chunk])
	
	var percent: int = 0
	if is_instance_valid(controller.progress_bar) and controller.progress_bar.max_value > 0:
		percent = int(controller.progress_bar.value / controller.progress_bar.max_value * 100.0)

	if percent != controller.last_percent and is_instance_valid(controller.status_label):
		if percent % 5 == 0 or percent == 100:
			controller.status_label.text = "Chunk (%d, %d) applied... %d%%" % [chunk_x, chunk_y, percent]
		
	controller.last_percent = percent

func _on_build_map_complete() -> void:
	var total_time_ms: int = Time.get_ticks_msec() - controller.generation_start_time_ms
	var total_time_s: float = float(total_time_ms) / 1000.0
	var tiles_per_second: float = 0.0
	if total_time_s > 0.0:
		tiles_per_second = float(controller.total_tiles_placed) / total_time_s
	
	var completion_msg: String = "Generation Complete! (%.2f seconds)" % total_time_s
	print("DEBUG: Build map complete →", completion_msg, "Tiles/sec:", tiles_per_second)

	if is_instance_valid(controller.status_label):
		controller.status_label.text = completion_msg
	if is_instance_valid(controller.generate_button):
		controller.generate_button.text = "IGNITION"
	if is_instance_valid(controller.tile_placement_time_label):
		controller.tile_placement_time_label.text = "Time: %.2fs" % total_time_s

	controller.utility.reset_system_state()
