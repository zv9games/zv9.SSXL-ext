# ssxl_chunk.gd
extends Node

# The reference to the Rust GDExtension class
var engine_ref: Node = null 

# Fired by main.gd to establish the communication link
func set_engine_reference(engine: Node) -> void:
	engine_ref = engine
	print("SSXLChunk: FFI engine reference set.")

# This is the function main.gd was calling (The PULL request)
func get_chunk_data(x: int, y: int) -> Dictionary:
	if not engine_ref:
		push_error("SSXLChunk: Engine reference is null. Cannot fetch data.")
		# FIX: Return an empty Dictionary ({}) instead of null to satisfy the return type.
		return {}

	# --- 🎯 THE FIX: Use the dynamic 'call()' method on the FFI object. ---
	# This avoids potential issues with Godot's FFI caching/binding when directly calling a method.
	# The string "fetch_chunk_data" MUST match the exact name registered in the Rust GDExtension.
	return engine_ref.call("fetch_chunk_data", x, y)
