extends Node
class_name SSXLMain

@export var world_width: int = 256
@export var world_height: int = 256
@export var chunk_size: int = 32
@export var auto_start: bool = true

@onready var conductor: SSXLConductor = $SSXLConductor
@onready var controller: SSXLController = $SSXLController
@onready var chunk_buffer: SSXLChunkBuffer = $SSXLChunkBuffer
@onready var renderer: SSXLRendererGD = $Node3D/SSXLRenderer

func _ready():
	# Aim the camera at the world origin
	$Camera3D.look_at(Vector3(0, 0, 0), Vector3.UP)

	print("Renderer world3d:", renderer.get_world_3d())
	print("Conductor has method:", conductor.has_method("start_generation"))
	print("Main: initializing SSXL pipeline...")

	# Controller ← Conductor
	conductor.conductor_ready.connect(controller._on_conductor_ready)
	conductor.generation_started.connect(controller._on_generation_started)
	conductor.generation_progress.connect(controller._on_generation_progress)
	conductor.chunk_rendered.connect(controller._on_chunk_rendered)
	conductor.generation_finished.connect(controller._on_generation_finished)
	conductor.generation_error.connect(controller._on_generation_error)
	conductor.debug_event.connect(controller._on_debug_event)
	conductor.ssxl_event.connect(controller._on_ssxl_event)

	# ChunkBuffer → Main
	chunk_buffer.chunk_ready.connect(_on_chunk_ready)
	chunk_buffer.chunk_updated.connect(_on_chunk_ready)

	# Renderer gets the chunk buffer
	renderer.set_chunk_buffer(chunk_buffer)

	print("Main: SSXL pipeline ready.")

	if auto_start:
		_start_demo_world()


func _start_demo_world():
	print("Main: Auto-starting demo world...")

	renderer.set_chunk_size(chunk_size)
	renderer.begin_world(world_width, world_height, chunk_size)

	# Wait one frame so renderer gets a Scenario RID
	await get_tree().process_frame

	conductor.start_generation(chunk_buffer)


func _on_chunk_ready(cx: int, cy: int):
	if not renderer.is_inside_tree():
		print("Renderer not ready yet, skipping chunk.")
		return

	if renderer.get_world_3d() == null:
		print("Renderer has no World3D, skipping chunk.")
		return

	renderer.apply_chunk(cx, cy)
