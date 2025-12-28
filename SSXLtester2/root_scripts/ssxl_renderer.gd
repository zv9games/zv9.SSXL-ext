extends SSXLRenderer
# NOTE: no class_name here, to avoid hiding the native SSXLRenderer

# All chunk application is now handled in Rust.
# These functions intentionally do nothing.

func _on_chunk_ready(cx: int, cy: int) -> void:
	pass

func _on_chunk_updated(cx: int, cy: int) -> void:
	pass

func get_chunk_instance(cx: int, cy: int) -> Node3D:
	return null
