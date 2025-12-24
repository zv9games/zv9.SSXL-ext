extends SSXLChunkBuffer
class_name SSXLChunkBufferGD

# Do NOT redeclare chunk_ready — it already exists in the native Rust class.
signal chunk_allocated(cx, cy)
signal buffer_reset

func _ready():
	print("SSXLChunkBufferGD: ready (Rust backend initialized).")
