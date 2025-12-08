extends SSXLEngine # CRITICAL: Ensure this extends SSXLEngine

# This script is attached to the SSXLEngine node, which is the custom GDExtension class.

# ============================================================================
# 1. LIFECYCLE CHECK
# ============================================================================

func _ready():
	# get_status() works because the script extends SSXLEngine.
	print("⚙️ [SSXLEngine] Core status on ready: %s" % get_status())
	if get_status() == "Uninitialized":
		push_error("SSXLEngine is uninitialized! Ensure main.gd called initialize_runtime_shell().")


# ============================================================================
# 2. PUBLIC API WRAPPERS (Passthrough to Native Methods)
# ============================================================================

func set_active_generator(name: String):
	"""Sets the generator ID the engine should use for new map requests."""
	set_generator(name)

func force_stop():
	"""Immediately halts any current map generation process."""
	stop_generation()
	print("⚠️ [SSXLEngine] Generation halted by user request.")

func get_chunk_data(x: int, y: int) -> Dictionary:
	"""Retrieves chunk data from the native cache."""
	return fetch_chunk_data(x, y)

func get_tile_count() -> int:
	"""Returns the total number of tiles currently placed on the TileMap."""
	return get_current_tile_count() as int

# FIX APPLIED HERE: Use call() to dynamically invoke the FFI CORE method.
func is_engine_active() -> bool:
	"""Checks if the core FFI engine thread is active (using dynamic call)."""
	# func is_active() -> bool) [FFI CORE]
	var result = call("is_active")
	if typeof(result) == TYPE_BOOL:
		return result
	
	# Fallback if the function is missing or returns null/void.
	return false
