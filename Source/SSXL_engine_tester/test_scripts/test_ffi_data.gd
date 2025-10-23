# test_ffi_data.gd
# Attached to the root Node in test_ffi_data.tscn
# Goal: Validate FFI Bridge Data Integrity (Phase 7 Finalization)

extends Node

# The GDExtension class exposed by the Rust side
const ENGINE_CLASS_NAME = "AetherionEngine"

func _ready():
	print("--- STARTING FFI DATA INTEGRITY TEST ---")
	
	# 1. Instantiate the GDExtension Class
	var engine = load_engine()
	if engine == null:
		print("ERROR: Godot failed to instantiate GDExtension class.")
		# Exit with a non-zero code to signal failure to the CLI
		get_tree().quit(1) 
		return

	# 2. Call the FFI-bound method and measure the data returned
	# We test key (1, 2, 3) to ensure arguments pass correctly.
	var chunk_data = engine.generate_chunk(1, 2, 3)
	
	# 3. Validation Checks
	var success = check_data_integrity(chunk_data)
	
	# 4. Final Quit
	if success:
		print("✅ FFI Data Integrity Test: PASSED.")
		get_tree().quit(0) # Signal success to the Rust CLI
	else:
		print("❌ FFI Data Integrity Test: FAILED.")
		get_tree().quit(1) # Signal failure to the Rust CLI

# Helper function to safely instantiate AetherionEngine
func load_engine():
	if not ClassDB.can_instantiate(ENGINE_CLASS_NAME):
		printerr("ERROR: GDExtension class '%s' not found. Ensure GDExtension is compiled and registered." % ENGINE_CLASS_NAME)
		return null
	return ClassDB.instantiate(ENGINE_CLASS_NAME)

# Core validation logic
func check_data_integrity(data):
	# A. Basic Structure Check
	if not data is Dictionary:
		printerr("FAIL: FFI return is not a Dictionary.")
		return false
	if not data.has("tiles") or not data.has("tile_count"):
		printerr("FAIL: FFI Dictionary is missing 'tiles' array or 'tile_count'.")
		return false
	
	var tiles = data.tiles
	var tile_count = data.tile_count
	
	print("INFO: Received chunk at key_x: %d with count: %d" % [data.get("key_x", -1), tile_count])
	
	# B. Non-empty Check (Assumes generation logic returns at least one tile)
	if tile_count == 0 or tiles.is_empty():
		printerr("FAIL: Chunk generation returned zero tiles. Check Rust core logic.")
		return false
		
	# C. Data Type and Value Check (Crucial for FFI integrity)
	var first_tile = tiles[0]
	
	# Validate types translated correctly across the FFI boundary
	if typeof(first_tile.id) != TYPE_INT:
		printerr("FAIL: Tile ID type mismatch. Expected TYPE_INT, got %s." % typeof(first_tile.id))
		return false
		
	if typeof(first_tile.level) != TYPE_FLOAT:
		printerr("FAIL: Tile Level type mismatch. Expected TYPE_FLOAT, got %s." % typeof(first_tile.level))
		return false
		
	print("INFO: First tile data verified (ID: %d, Level: %f)." % [first_tile.id, first_tile.level])
	
	# If all checks pass
	return true
