# main.gd – Fully fixed and panic-free
extends Node

@onready var engine: Node = $SSXLEngine
@onready var signals: Node = $SSXLSignals
@onready var presenter: Node = $SSXLChunk
@onready var tilemap: TileMap = $SSXLTilemap
# 🚨 ENSURE THIS PATH IS CORRECT (e.g., $Camera2D or $../Camera2D)
@onready var camera: Camera2D = $Camera2D

func _ready() -> void:
	print("SSXL Engine Startup...")

	# 1. Wire Godot nodes to Rust
	# --- CRITICAL SAFETY CHECK ---
	if !is_instance_valid(presenter):
		push_error("FATAL ERROR: The node '$SSXLChunk' (Presenter) is missing. Please add an SSXLChunk node to the scene.")
		return
	# -----------------------------
	
	# FIX 1: Call the correct GDScript function on the presenter node.
	presenter.set_engine_reference(engine)
	
	# FIX 2 & 3: Removed non-existent FFI calls (set_tilemap, set_signals_node).

	# 2. Initialize channels & state
	# 🎯 FINAL FIX: Pass the 'signals' Node reference, fulfilling the 1-argument requirement.
	engine.initialize_runtime_shell(signals)

	# 3. Now spawn the actual conductors
	engine.start_async_conductors()       # ← Must come AFTER initialize_runtime_shell!

	# 4. Connect signals
	signals.chunk_data_ready.connect(_on_chunk_data_ready)
	signals.build_map_start.connect(func(): print("Build started"))
	signals.build_map_complete.connect(func(): print("Build complete"))

	# Give threads time to start
	await get_tree().create_timer(0.1).timeout

	_start_generation()


func _process(_delta: float) -> void:
	# This function is assumed to be exposed by Rust and is crucial for polling messages.
	engine.tick(Time.get_ticks_msec())


func _start_generation() -> void:
	var cfg = tilemap.get_generation_config()
	
	var w: int = cfg.width
	var h: int = cfg.height
	var gen: String = cfg.generator

	cfg.erase("generator")
	cfg["generator_name"] = gen

	# --- Camera Fixes ---
	var tile_size: int = tilemap.tile_set.tile_size.x
	var center_x: float = float(w) * tile_size / 2.0
	var center_y: float = float(h) * tile_size / 2.0
	
	# Only execute if the camera node was successfully found
	if is_instance_valid(camera):
		camera.global_position = Vector2(center_x, center_y)

		var viewport_width: float = get_viewport().get_visible_rect().size.x
		var map_pixel_width: float = float(w) * tile_size
		var required_zoom: float = viewport_width / map_pixel_width
		
		camera.zoom = Vector2.ONE * max(required_zoom, 0.005) # Max zoom-out is now 0.005
	# --- End Camera Fixes ---
	
	tilemap.clear()
	# These remaining engine calls are core functions and should be exposed by Rust
	engine.clear_completed_chunks()
	engine.set_generator(gen)
	engine.build_map(cfg)

	print("Generation started: %dx%d..." % [w, h])


# PULL-based rendering (clean & simple)
func _on_chunk_data_ready(x: int, y: int) -> void:
	call_deferred("_render_chunk", x, y)

func _render_chunk(x: int, y: int) -> void:
	# The PULL request now correctly goes through the engine node
	var data: Dictionary = engine.fetch_chunk_data(x, y)
	if data.is_empty():
		return

	# --- Type and Coordinate Correction ---
	var world_offset: Vector2i = Vector2i(x, y)
	
	var raw_positions: Variant = data.get("positions", PackedVector2Array())
	var raw_atlas_coords: Variant = data.get("atlas_coords", PackedVector2Array())
	
	var absolute_positions: Array[Vector2i] = []
	
	# 1. Correct Positions 
	if raw_positions is PackedInt32Array:
		var i = 0
		var raw_int_array: PackedInt32Array = raw_positions
		while i < raw_int_array.size():
			if raw_int_array.size() > i + 1:
				var rel_pos: Vector2i = Vector2i(raw_int_array[i], raw_int_array[i+1])
				absolute_positions.append(rel_pos + world_offset)
			i += 2 # <-- This index increment is CORRECT for iterating over (x, y) pairs!
	
	elif raw_positions is PackedVector2Array or raw_positions is Array:
		for rel_pos_v in raw_positions:
			var rel_pos: Vector2i = Vector2i(rel_pos_v)
			absolute_positions.append(rel_pos + world_offset)

	
	# 2. Correct Atlas Coords 
	var final_atlas_coords: Array[Vector2i] = []
	
	if raw_atlas_coords is PackedInt32Array:
		var i = 0
		var raw_int_array: PackedInt32Array = raw_atlas_coords
		while i < raw_int_array.size():
			if raw_int_array.size() > i + 1:
				final_atlas_coords.append(Vector2i(raw_int_array[i], raw_int_array[i+1]))
			i += 2 # <-- This index increment is CORRECT for iterating over (x, y) pairs!
	
	elif raw_atlas_coords is PackedVector2Array or raw_atlas_coords is Array:
		for coord_v in raw_atlas_coords:
			final_atlas_coords.append(Vector2i(coord_v))
	
	
	data["positions"] = absolute_positions
	data["atlas_coords"] = final_atlas_coords
	# --- End Correction ---

	if absolute_positions.is_empty():
		return

	tilemap.batch_set_tiles_v4(data)
	print("Chunk (%d, %d) rendered" % [x, y])
