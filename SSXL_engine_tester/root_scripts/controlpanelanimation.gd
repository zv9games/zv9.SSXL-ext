class_name control_panel_animation_logic
extends RefCounted

var controller
var tile_flip_queue: Array = []
const MAX_FLIPS_PER_TICK: int = 1000
var animation_is_active: bool = false

func _init(p_controller: Control) -> void:
	controller = p_controller

func _on_animate_button_pressed() -> void:
	animation_is_active = !animation_is_active
	var button_pressed: bool = animation_is_active
	
	if is_instance_valid(controller.ssxl_engine) and controller.ssxl_engine.has_method("set_animation_enabled"):
		controller.ssxl_engine.set_animation_enabled(button_pressed)
	
	setup_animation_worker(button_pressed)

func setup_animation_worker(should_animate: bool) -> void:
	animation_is_active = should_animate
	
	if is_instance_valid(controller.animation_timer):
		controller.animation_timer.stop()
	if is_instance_valid(controller.engine_timer):
		controller.engine_timer.stop()
		
	if should_animate:
		if is_instance_valid(controller.animation_timer):
			controller.animation_timer.start()
	else:
		if is_instance_valid(controller.engine_timer):
			controller.engine_timer.call_deferred("start")

func _on_animation_timer_timeout() -> void:
	if not controller.is_generating or not is_instance_valid(controller.ssxl_tilemap):
		return
		
	var batch_size: int = min(tile_flip_queue.size(), MAX_FLIPS_PER_TICK)
	
	for i in range(batch_size):
		var data: Dictionary = tile_flip_queue.pop_front()
		if not data.is_empty():
			process_tile_flip(data.tile_id, data.flip_frame)
		
	controller.ssxl_tilemap.force_update()
	
	if is_instance_valid(controller.ssxl_oracle):
		controller.ssxl_oracle.tick()

func _on_engine_timer_timeout() -> void:
	controller.engine_tick_count += 1
	if is_instance_valid(controller.ssxl_oracle):
		controller.ssxl_oracle.tick()
	
	if is_instance_valid(controller.engine_timer_label):
		controller.engine_timer_label.text = "Tick Count: %d" % controller.engine_tick_count

func process_tile_flip(tile_id: int, flip_frame: int) -> void:
	if not is_instance_valid(controller.ssxl_tilemap) or not is_instance_valid(controller.grid_width):
		return

	var map_width: int = int(controller.grid_width.value)
	if map_width <= 0:
		return

	var x: int = tile_id % map_width
	var y: int = tile_id / map_width
	var coords: Vector2i = Vector2i(x, y)
	
	var source_id: int = controller.BASE_SOURCE_ID
	var atlas_coords: Vector2i = controller.BASE_ATLAS_COORDS
	
	controller.ssxl_tilemap.set_cell(0, coords, source_id, atlas_coords, flip_frame)

func _on_tile_flip_updated(tile_id: int, flip_frame: int) -> void:
	if not animation_is_active or not controller.is_generating:
		return

	tile_flip_queue.append({ "tile_id": tile_id, "flip_frame": flip_frame })

func redraw_tilemap_throttled() -> void:
	if is_instance_valid(controller.ssxl_tilemap):
		controller.ssxl_tilemap.force_update()
