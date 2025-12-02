extends Node

# Ensure this matches your GDExtension class name
const ENGINE_CLASS_NAME = "SSXLEngine"

# --- Utility Functions ---

# Loads the SSXLEngine GDExtension instance.
func create_engine_instance():
    if not ClassDB.can_instantiate(ENGINE_CLASS_NAME):
        printerr("FATAL: GDExtension is not compiled or registered.")
        return null
    return ClassDB.instantiate(ENGINE_CLASS_NAME)

# Function to explicitly format and print the result
# CRITICAL: This uses 'print' (stdout) for Rust to capture the result.
func print_test_result(test_name, success, reason):
    if success:
        print("✅ GDSCIPT TEST PASS: %s" % test_name)
    else:
        # Use printerr for detailed failures, still captured by Rust's stderr handler.
        printerr("❌ GDSCIPT TEST FAIL: %s | Reason: %s" % [test_name, reason])

# Core validation logic
func check_data_integrity(data):
    if not data is Dictionary:
        printerr("FAIL: FFI return is not a Dictionary.")
        return false
    if not data.has("tiles") or not data.has("tile_count"):
        printerr("FAIL: FFI Dictionary is missing 'tiles' array or 'tile_count'.")
        return false
    
    var tiles = data.tiles
    var tile_count = data.tile_count
    
    # Informative log to help debug generation issues
    print("INFO: Received chunk data. Tile count: %d" % tile_count)
    
    if tile_count == 0 or tiles.is_empty():
        printerr("FAIL: Chunk generation returned zero tiles. Check Rust core logic.")
        return false
        
    # Add more detailed checks here (e.g., data types for 'id' and 'level')
    if typeof(tiles[0].id) != TYPE_INT or typeof(tiles[0].level) != TYPE_FLOAT:
        printerr("FAIL: Tile data type mismatch detected across FFI boundary.")
        return false
        
    return true

# --- NEW FUNCTION: Defer the quit to ensure stdout flush ---
# This is the most reliable way to guarantee that all print() calls 
# are processed by the engine before the process terminates.
func quit_safe(code: int):
    # Wait one process frame to ensure all deferred print calls are flushed
    await get_tree().process_frame 
    get_tree().quit(code)

# --- CORE TEST RUNNER ---

func _ready():
    # 1. Instantiate the Rust Engine
    var engine = create_engine_instance()
    
    var test_passed = false
    var failure_reason = "GDExtension instantiation failed."
    var exit_code = 1 # Assume failure by default
    
    if engine == null:
        # Handle instantiation failure immediately
        print_test_result("Generation Integration Test", test_passed, failure_reason)
        # exit_code remains 1
    else:
        # 2. Run the actual generation test logic
        failure_reason = "Unknown error."
        var test_x = 100
        var test_y = 200
        
        print("TEST: Requesting generation for chunk (%d, %d)..." % [test_x, test_y])
        
        # Call the FFI function from Rust
        var chunk_data = engine.request_generation(test_x, test_y) 
        
        if chunk_data:
            if check_data_integrity(chunk_data):
                test_passed = true
                exit_code = 0 # Success
                failure_reason = "N/A"
            else:
                failure_reason = "Data integrity check failed (see errors above)."
        else:
            failure_reason = "FFI call returned null or an invalid result."

        # 3. Report Final Status
        print_test_result("Generation Integration Test", test_passed, failure_reason)
    
    # 4. Defer the exit command to the safe function
    call_deferred("quit_safe", exit_code)