class_name control_panel_gen_logic
extends RefCounted

var control_panel: Control

func _init(panel: Control) -> void:
	control_panel = panel

func start_generation() -> void:
	if control_panel.is_generating:
		stop_generation()
		return
	
	_setup_generation_start()
	
	var width: int = int(control_panel.grid_width.value)
	var height: int = int(control_panel.grid_height.value)
	var seed_text: String = control_panel.seed_input.text
	var generator_name: String = control_panel.tile_type_selector.get_item_text(
		control_panel.tile_type_selector.selected
	)
	var seed_value: int = _parse_seed(seed_text)
	
	if not _validate_inputs(width, height):
		if is_instance_valid(control_panel.utility):
			control_panel.utility.stop_generation()
		return
	
	if is_instance_valid(control_panel.ssxl_engine):
		control_panel.ssxl_engine.set_generator(generator_name)
		# Defer to avoid stalling UI while Rust starts
		call_deferred("_start_ffi_generation", width, height, str(seed_value), generator_name)
		control_panel.status_label.text = "Generating map..."
	else:
		if is_instance_valid(control_panel.utility):
			control_panel.utility.stop_generation()

func _start_ffi_generation(width: int, height: int, seed: String, generator_name: String) -> void:
	if is_instance_valid(control_panel.ssxl_engine):
		control_panel.ssxl_engine.build_map(width, height, seed, generator_name)

func stop_generation() -> void:
	# FIX: use utility (not 'quality')
	if is_instance_valid(control_panel.utility):
		control_panel.utility.stop_generation()
	if is_instance_valid(control_panel.ssxl_engine):
		control_panel.ssxl_engine.stop_generation()

# SSXLTileMap rendering
func _on_chunk_data_ready(key_x: int, key_y: int) -> void:
	print("CHUNK RECEIVED → (", key_x, ",", key_y, ")")

	if not is_instance_valid(control_panel.presenter):
		push_error("FATAL: control_panel.presenter is NULL! Add it in control_panel.gd")
		return
	if not is_instance_valid(control_panel.ssxl_tilemap):
		push_error("FATAL: ssxl_tilemap missing or invalid — cannot render chunk.")
		return

	var chunk_opt = control_panel.presenter.get_chunk_data(key_x, key_y)
	if chunk_opt.is_none():
		return

	var chunk_data: Dictionary = chunk_opt.unwrap()
	if not control_panel.utility.check_data_integrity(chunk_data):
		push_warning("Chunk failed integrity: %s, %s" % [key_x, key_y])
		return

	var tiles: Array = chunk_data.tiles
	var layer: int = 0
	var source_id: int = control_panel.BASE_SOURCE_ID
	var atlas_coords: Vector2i = control_panel.BASE_ATLAS_COORDS
	var alt_id: int = control_panel.BASE_ALT_ID

	var placed: int = 0
	for tile in tiles:
		var world_x: int = key_x * control_panel.CHUNK_SIZE_X + tile.x
		var world_y: int = key_y * control_panel.CHUNK_SIZE_Y + tile.y
		var coords := Vector2i(world_x, world_y)

		# FIX: Godot 4 signature — set_cell(layer, coords, source_id, atlas_coords, alt_id)
		control_panel.ssxl_tilemap.set_cell(layer, coords, source_id, atlas_coords, alt_id)

		control_panel.total_tiles_placed += 1
		placed += 1

	# Optional: force update for immediate visual refresh
	control_panel.ssxl_tilemap.force_update()

	control_panel.progress_bar.value = control_panel.total_tiles_placed
	control_panel.signal_handler._on_chunk_generated(key_x, key_y, placed)

func _setup_generation_start() -> void:
	control_panel.is_generating = true
	control_panel.generate_button.text = "STOP GEN (F5)"
	control_panel.generation_start_time_ms = Time.get_ticks_msec()
	control_panel.total_tiles_placed = 0

	var map_width: int = int(control_panel.grid_width.value)
	var map_height: int = int(control_panel.grid_height.value)
	control_panel.progress_bar.max_value = float(map_width * map_height)
	control_panel.progress_bar.value = 0.0
	control_panel.progress_bar.visible = true

	if is_instance_valid(control_panel.ssxl_tilemap):
		control_panel.ssxl_tilemap.clear()

	# Timer orchestration
	if control_panel.animation_logic.animation_is_active:
		control_panel.engine_timer.stop()
		control_panel.animation_timer.start()
	else:
		control_panel.animation_timer.stop()

	if is_instance_valid(control_panel.engine_timer):
		control_panel.engine_timer.stop()
		control_panel.engine_timer.call_deferred("start")

func _validate_inputs(map_width: int, map_height: int) -> bool:
	if map_width <= 0 or map_height <= 0:
		return false
	if map_width % control_panel.CHUNK_SIZE_X != 0 or map_height % control_panel.CHUNK_SIZE_Y != 0:
		push_warning("Map size must be multiple of chunk size!")
		return false
	return true

func _parse_seed(seed_text: String) -> int:
	if seed_text.is_empty():
		return Time.get_ticks_msec()
	return seed_text.strip_edges().to_lower().hash()
