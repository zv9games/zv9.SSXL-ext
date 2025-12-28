extends Node
class_name SSXLMain

@onready var controller: SSXLController = $SSXLController
@onready var conductor: SSXLConductor = $SSXLConductor
@onready var renderer: SSXLRenderer = $Node3D/SSXLRenderer
@onready var chunk_buffer: SSXLChunkBuffer = $SSXLChunkBuffer   # still exists, but no longer used by renderer

@export var world_w: int = 100
@export var world_h: int = 100
@export var chunk_size: int = 32

# Optional: user-assigned material (no longer required for renderer)
@export var world_material: Material

var _gen_start_ms: int = 0

func _ready() -> void:
	print("Main: initializing SSXL pipeline...")
	print("Main: renderer type =", typeof(renderer), " class_name =", renderer.get_class())

	# Conductor → Controller signals
	conductor.conductor_ready.connect(controller._on_conductor_ready)
	conductor.generation_started.connect(_on_generation_started)
	conductor.generation_progress.connect(controller._on_generation_progress)
	conductor.generation_finished.connect(self._on_generation_finished)
	conductor.generation_error.connect(controller._on_generation_error)

	print("Main: SSXL pipeline ready.")
	_start_demo_world()


func _start_demo_world() -> void:
	print("Main: Auto-starting demo world...")

	controller.setup_world_metrics(world_w, world_h)

	# IMPORTANT:
	# Do NOT override the renderer material here unless explicitly provided.
	if world_material != null:
		renderer.set_material(world_material)

	# Initialize renderer world
	renderer.begin_world(world_w, world_h, chunk_size)

	# Register the RENDERER as the tilemap/render target (CRITICAL)
	conductor.set_tilemap(renderer)

	var config: Dictionary = {
		"width": world_w,
		"height": world_h,
		"chunk_size": chunk_size
	}

	var ok_build: bool = conductor.build_map(config)
	if not ok_build:
		push_error("Main: conductor.build_map(config) failed.")
		return

	var ok: bool = conductor.start_generation(renderer)
	if not ok:
		push_error("Main: conductor.start_generation(renderer) returned false.")


# Accept the signal’s arguments, but ignore them.
func _on_generation_started(_a = null, _b = null) -> void:
	_gen_start_ms = Time.get_ticks_msec()
	print("Main: Generation started at", _gen_start_ms, "ms.")


func _on_generation_finished(tilemap_id: int) -> void:
	var end_ms: int = Time.get_ticks_msec()
	var elapsed_ms: int = max(1, end_ms - _gen_start_ms)

	# Convert to minutes + seconds
	var total_seconds: float = float(elapsed_ms) / 1000.0
	var minutes: int = int(total_seconds) / 60
	var seconds: float = total_seconds - float(minutes * 60)

	var total_tiles: int = world_w * world_h

	# Chunks needed to cover the world
	var chunks_x: int = (world_w + chunk_size - 1) / chunk_size
	var chunks_y: int = (world_h + chunk_size - 1) / chunk_size
	var total_chunks: int = chunks_x * chunks_y

	var tiles_per_ms: float = float(total_tiles) / float(elapsed_ms)
	var tiles_per_sec: float = tiles_per_ms * 1000.0

	print("Main: Generation finished. Building chunk meshes...")
	print("Main: All chunks applied.")

	print("---- SSXL Metrics ----")
	print("World size: ", world_w, "x", world_h, " tiles")
	print("Chunk size: ", chunk_size, "x", chunk_size, " tiles")
	print("Chunks:     ", chunks_x, "x", chunks_y, " = ", total_chunks)
	print("Total tiles:", total_tiles)
	print("Time:       ", elapsed_ms, " ms (", minutes, " min ", seconds, " sec )")
	print("Tiles/ms:   ", tiles_per_ms)
	print("Tiles/sec:  ", tiles_per_sec)
	print("----------------------")
