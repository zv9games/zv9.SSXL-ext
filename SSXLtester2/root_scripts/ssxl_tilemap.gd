extends TileMap

const CHUNK_SIZE := 32
const RADIUS := 8

var last_instance_id := -1   # ✅ throttle repeated prints


func _ready():
	var id := get_instance_id()

	# ✅ Only print if the ID changed (Option 2 throttling)
	if id != last_instance_id:
		last_instance_id = id
		print("TileMap instance ID =", id)

	await get_tree().process_frame
	_preallocate_chunks()


func _preallocate_chunks():
	for cy in range(-RADIUS, RADIUS + 1):
		for cx in range(-RADIUS, RADIUS + 1):
			var cell_x := cx * CHUNK_SIZE + 1
			var cell_y := cy * CHUNK_SIZE + 1
			set_cell(0, Vector2i(cell_x, cell_y), 0)


func get_raw_chunk_data_ptr(layer: int, cx: int, cy: int) -> int:
	return 0


func notify_chunk_data_changed(layer: int, cx: int, cy: int) -> void:
	pass
