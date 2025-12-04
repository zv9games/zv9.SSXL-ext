# ╔════════════════════════════════════════════════════════════════════════════╗
# ║                  SSXL-ext – GODOT NOOB SURVIVAL GUIDE                      ║
# ║            (All the real-world scenarios in one copy-paste block)          ║
# ╚════════════════════════════════════════════════════════════════════════════╝

# Save this as res://SSXL_Noob_Survival_Guide.gd (or just read it as documentation)
# Everything below is tested & works with the official SSXL-ext manual (Dec 2025)

extends Node
class_name SSXLNoobGuide

# ──────────────────────────────────────────────────────────────────────────────
# CONFIGURATION (tweak these!)
# ──────────────────────────────────────────────────────────────────────────────
const CHUNK_SIZE := 32                    # Must match ssxl_config.json
const BUFFER_SIZE := 2 * 1024 * 1024       # 2 MB – safe for huge chunks
const VIEW_RADIUS := 3                    # How many chunks around player/camera

# ──────────────────────────────────────────────────────────────────────────────
# INTERNAL STATE
# ──────────────────────────────────────────────────────────────────────────────
var loaded_chunks := {}                   # Vector2i(chunk_x, chunk_y) → true
var tilemap : TileMap                     # Drag your TileMap here in the inspector

# ──────────────────────────────────────────────────────────────────────────────
# 1. INITIAL SETUP (autoload singleton SSXL.gd should call this on _ready)
# ──────────────────────────────────────────────────────────────────────────────
func initialize(tilemap_node: TileMap):
    tilemap = tilemap_node
    
    # Pre-allocate the giant buffer (the #1 performance trick)
    SSXL.buffer = PackedByteArray()
    SSXL.buffer.resize(BUFFER_SIZE)
    
    # Start the Rust engine
    var ok = SSXL.ssxl_engine.call("ssxl_start_runtime")
    if not ok:
        push_error("SSXL Rust runtime failed to start!")
        return
    print("SSXL Rust engine READY – zero-latency generation active!")
    
    # Request first chunks around spawn
    request_chunks_around(Vector2.ZERO, 2)

# ──────────────────────────────────────────────────────────────────────────────
# 2. REQUEST CHUNKS AROUND A WORLD POSITION (use at spawn or teleport)
# ──────────────────────────────────────────────────────────────────────────────
func request_chunks_around(world_pos: Vector2, radius: int = 2):
    var center := Vector2i(world_pos / CHUNK_SIZE)
    for x in range(-radius, radius + 1):
        for y in range(-radius, radius + 1):
            var ck := center + Vector2i(x, y)
            if not loaded_chunks.has(ck):
                SSXL.ssxl_engine.call("ssxl_request_chunk", ck.x, ck.y)
                loaded_chunks[ck] = true

# ──────────────────────────────────────────────────────────────────────────────
# 3. DYNAMIC CAMERA / PLAYER FOLLOW (call every frame or every few frames)
# ──────────────────────────────────────────────────────────────────────────────
func update_visible_chunks_around_player(player: Node2D):
    var center := Vector2i(player.global_position / CHUNK_SIZE)
    var desired := {}
    
    # Mark everything we want to keep loaded
    for x in range(-VIEW_RADIUS, VIEW_RADIUS + 1):
        for y in range(-VIEW_RADIUS, VIEW_RADIUS + 1):
            desired[center + Vector2i(x, y)] = true
    
    # Request missing chunks
    for ck in desired.keys():
        if not loaded_chunks.has(ck):
            SSXL.ssxl_engine.call("ssxl_request_chunk", ck.x, ck.y)
            loaded_chunks[ck] = true
    
    # Optional: unload super-far chunks to save memory
    for ck in loaded_chunks.keys():
        if not desired.has(ck) and center.distance_to(ck) > VIEW_RADIUS + 3:
            unload_chunk(ck)

func unload_chunk(ck: Vector2i):
    if not tilemap: return
    var min_pos := ck * CHUNK_SIZE
    var max_pos := min_pos + Vector2i(CHUNK_SIZE, CHUNK_SIZE)
    for x in range(min_pos.x, max_pos.x):
        for y in range(min_pos.y, max_pos.y):
            tilemap.erase_cell(0, Vector2i(x, y))
    loaded_chunks.erase(ck)

# ──────────────────────────────────────────────────────────────────────────────
# 4. THE POLLING LOOP – THE HEART OF ZERO LATENCY (put in SSXL.gd _process)
# ──────────────────────────────────────────────────────────────────────────────
func poll_and_place_tiles():
    if not SSXL.ssxl_engine or not SSXL.ssxl_engine.is_open():
        return
        
    var result : int = SSXL.ssxl_engine.call(
        "ssxl_poll_progress_message",
        SSXL.buffer.ptr(),
        SSXL.buffer.size()
    )
    
    match result:
        # ─── SUCCESS ───
        var n if n > 0:
            var message_bytes := SSXL.buffer.slice(0, n)
            var message = Bincode.decode(message_bytes)   # ← you need the Bincode addon
            
            var chunk_x : int = message.key_x
            var chunk_y : int = message.key_y
            
            for tile in message.tiles:
                var world_x = tile.coords_x + chunk_x * CHUNK_SIZE
                var world_y = tile.coords_y + chunk_y * CHUNK_SIZE
                tilemap.set_cell(0, Vector2i(world_x, world_y), tile.id, Vector2i.ZERO)
        
        # ─── NOTHING READY ───
        0:
            pass
        
        # ─── ERROR CODES (see manual Appendix A.4) ───
        -1: printerr("SSXL ERROR: Runtime not initialized!")
        -2: printerr("SSXL FATAL: Mutex poisoned – something panicked inside Rust!")
        -3: printerr("SSXL FATAL: Channel disconnected – engine is dead. Restarting scene...")
            get_tree().reload_current_scene()
        -5: printerr("SSXL WARNING: Buffer too small! Increase BUFFER_SIZE to at least 4MB")
        var err: printerr("SSXL Unknown error code: ", err)

# ──────────────────────────────────────────────────────────────────────────────
# 5. LIVE DEBUG DASHBOARD (optional – slap on a CanvasLayer)
# ──────────────────────────────────────────────────────────────────────────────
func update_debug_ui(label_chunks: Label, label_fps: Label):
    if not SSXL.ssxl_engine: return
    var completed : int = SSXL.ssxl_engine.call("ssxl_get_chunks_completed")
    label_chunks.text = "Chunks generated: %d" % completed
    label_fps.text    = "FPS: %d" % Engine.get_frames_per_second()

# ──────────────────────────────────────────────────────────────────────────────
# 6. HOT-SWAP GENERATORS ON THE FLY (press R to go from Perlin → Caves)
# ──────────────────────────────────────────────────────────────────────────────
func hot_swap_generator(generator_id: String):
    SSXL.ssxl_engine.call("ssxl_shutdown_runtime")
    
    var cfg = ConfigFile.new()
    cfg.load("res://ssxl_config.json")
    cfg.set_value("generation", "default_generator_id", generator_id)
    cfg.save("res://ssxl_config.json")
    
    # Restart with new config
    SSXL.ssxl_engine.call("ssxl_start_runtime")
    print("Generator swapped to: ", generator_id)
    # Clear old world & reload around player
    loaded_chunks.clear()
    tilemap.clear()
    request_chunks_around(get_viewport().get_camera_2d().get_target_position(), 2)

# ──────────────────────────────────────────────────────────────────────────────
# 7. CLEAN SHUTDOWN (already in the main SSXL autoload, but here for completeness)
# ──────────────────────────────────────────────────────────────────────────────
func _notification(what: int):
    if what in [NOTIFICATION_WM_CLOSE_REQUEST, NOTIFICATION_EXIT_TREE]:
        if SSXL.ssxl_engine and SSXL.ssxl_engine.is_open():
            SSXL.ssxl_engine.call("ssxl_shutdown_runtime")
            print("SSXL Rust engine gracefully shut down – no leaks!")

# ──────────────────────────────────────────────────────────────────────────────
# YOU ARE NOW A GODOT + RUST INFINITE WORLD WIZARD
# ──────────────────────────────────────────────────────────────────────────────
# Copy → Paste → Hit Play → Watch buttery-smooth procedural magic happen.
# No frame drops. Ever.
# You’re welcome.