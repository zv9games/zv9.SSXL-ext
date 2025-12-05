# camera.gd
extends Camera2D
class_name SSXLCameraDriver

const CHUNK_SIZE := 32
const VIEW_RADIUS := 3
var last_chunk_key := Vector2i(-999, -999)

func _physics_process(delta):
	# 🧭 O(1) current chunk key check
	var current_chunk_key = world_to_chunk(global_position)
	
	if current_chunk_key != last_chunk_key:
		last_chunk_key = current_chunk_key
		request_chunks_around(current_chunk_key)

# 🗺️ O(R^2) dispatch (R=VIEW_RADIUS)
func request_chunks_around(center_key: Vector2i):
	var signals = get_node("/root/SSXLSignals")
	
	for x in range(-VIEW_RADIUS, VIEW_RADIUS + 1):
		for y in range(-VIEW_RADIUS, VIEW_RADIUS + 1):
			var key_to_request = center_key + Vector2i(x, y)
			# O(1) signal dispatch
			signals.chunk_request.emit(key_to_request)

# 📐 O(1) coordinate conversion
func world_to_chunk(world_pos: Vector2) -> Vector2i:
	return Vector2i(floor(world_pos.x / CHUNK_SIZE), floor(world_pos.y / CHUNK_SIZE))
