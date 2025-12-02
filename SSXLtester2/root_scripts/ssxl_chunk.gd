# ssxl_chunk.gd
extends Node

var engine_ref: Node = null

func set_engine_reference(engine: Node) -> void:
	engine_ref = engine
	print("SSXLChunk: FFI engine reference set.")

func get_chunk_data(x: int, y: int) -> Dictionary:
	if not engine_ref:
		push_error("SSXLChunk: Engine reference is null.")
		return {}

	# Call the Rust fetch_chunk_data (must be the new one returning positions/atlas_coords)
	var result: Variant = engine_ref.call("fetch_chunk_data", x, y)

	# Safety: ensure we really got a Dictionary back
	if typeof(result) != TYPE_DICTIONARY:
		push_error("SSXLChunk: fetch_chunk_data returned wrong type: %s" % type_string(typeof(result)))
		return {}

	return result as Dictionary
