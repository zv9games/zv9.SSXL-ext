# engine.gd
extends Node

const BUFFER_SIZE := 2 * 1024 * 1024 
var ffi: Node = self # Assumes this Node IS the GDExtension interface

func _ready():
	# 🚀 O(1) start of Rust Parallel Backbone
	if not ffi.call("ssxl_start_runtime", BUFFER_SIZE):
		push_error("SSXL FFI start failed.")

func _process(delta):
	# ♻️ O(1) non-blocking poll for results
	var result_bytes = ffi.call("ssxl_poll_result")
	
	if result_bytes.size() > 0:
		# ⚠️ O(N) Bincode decode (unavoidable deserialization)
		var msg = Bincode.decode(result_bytes) 
		var key = Vector2i(msg.key_x, msg.key_y)
		# O(1) signal dispatch
		get_node("/root/SSXLSignals").chunk_loaded.emit(key, msg)

func request_chunk(chunk_key: Vector2i):
	# 📡 O(1) request to the Rust Conductor
	ffi.call("ssxl_request_chunk", chunk_key.x, chunk_key.y)

func _notification(what):
	if what in [NOTIFICATION_WM_CLOSE_REQUEST, NOTIFICATION_EXIT_TREE]:
		# 🛑 O(1) graceful shutdown
		if ffi and ffi.is_open():
			ffi.call("ssxl_shutdown_runtime")
