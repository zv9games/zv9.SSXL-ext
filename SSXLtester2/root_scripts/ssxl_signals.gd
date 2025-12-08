extends Node

# ============================================================================
# 1. CUSTOM SIGNAL DEFINITION (Consumed by this script's handler)
# ============================================================================

# Signal: Emitted by the Camera2D script, contains the coordinate of the chunk to generate.
signal chunk_request(coord: Vector2i) 


# ============================================================================
# 2. LIFECYCLE & CORE SIGNAL CONNECTION
# ============================================================================

@onready var ssxl_engine = get_parent().get_node("SSXLEngine")

func _ready():
	if ssxl_engine:
		# A. Connect our *own* custom signal to a method in this script.
		# This transforms a GDScript event into a native core command.
		self.chunk_request.connect(self._on_chunk_request)

		# B. Connect ALL native signals from the Rust core.
		# Since this SSXLSignals node instance was passed to the Engine core's 
		# 'initialize_runtime_shell', the signals are emitted by 'self'.
		
		# Generation Lifecycle (using original snake_case names)
		self.connect("build_map_start", Callable(self, "build_map_start"))
		self.connect("chunk_data_updated", Callable(self, "chunk_data_updated"))
		self.connect("chunk_generated_batch", Callable(self, "chunk_generated_batch"))
		self.connect("build_map_complete", Callable(self, "build_map_complete"))
		self.connect("build_map_stopped", Callable(self, "build_map_stopped"))
		self.connect("generation_error", Callable(self, "generation_error"))
		self.connect("chunk_data_ready", Callable(self, "chunk_data_ready"))
		self.connect("tick_complete", Callable(self, "tick_complete"))
		self.connect("chunk_applied", Callable(self, "chunk_applied"))
		
		# Animation & Utility
		self.connect("tile_flip_updated", Callable(self, "tile_flip_updated"))
		self.connect("animation_update", Callable(self, "animation_update"))
		self.connect("animation_state_changed", Callable(self, "animation_state_changed"))
		self.connect("engine_status_updated", Callable(self, "engine_status_updated"))
		
		print("Signals: Connected all GDExtension broadcasts successfully.")
	else:
		push_error("CRITICAL: SSXLEngine node not found! Engine communication disabled.")

# ============================================================================
# 3. REQUEST HANDLER (Transforms GDScript signal into native call)
# ============================================================================

func _on_chunk_request(coord: Vector2i):
	if ssxl_engine:
		var generator_id = ssxl_engine.get_active_generator_id()
		
		# 1. Dispatch the request to the Engine Core.
		# func build_map_by_size(width: i32, height: i32, _generator_id: GString)) [GDExt]
		# The engine is configured to treat the coordinate as a 1x1 area start 
		# for generating a single chunk at that key.
		ssxl_engine.build_map_by_size(coord.x, coord.y, generator_id)


# ============================================================================
# 4. SIGNAL HANDLERS (Listening to Native Broadcasts)
# ============================================================================

# signal build_map_start() [signals.rs]
func build_map_start():
	print("🔔 [Signal] Map build started.")

# signal chunk_data_updated(x: i32, y: i32) [signals.rs]
func chunk_data_updated(x: int, y: int):
	# Data has been calculated but not yet applied to the TileMap.
	# SSXLChunk script would typically listen to this or chunk_data_ready
	# and call ssxl_engine.fetch_chunk_data(x, y) if manual application is desired.
	pass

# signal chunk_data_ready(x: i32, y: i32) [signals.rs]
func chunk_data_ready(x: int, y: int):
	print("💾 [Signal] Chunk Data Ready in cache at (%d, %d)." % [x, y])

# signal chunk_generated_batch(batch: Dictionary) [signals.rs]
func chunk_generated_batch(batch: Dictionary):
	# Used when the engine core delivers map data as a batch for performance.
	print("📦 [Signal] Received a batch of %d chunks." % batch.size())
	
# signal chunk_applied(key_x: i64, key_y: i64) [lib.rs]
func chunk_applied(key_x: int, key_y: int):
	print("✨ [Signal] Chunk data successfully rendered by the FFI Core at (%d, %d)." % [key_x, key_y])

# signal build_map_complete() [signals.rs]
func build_map_complete():
	print("✅ [Signal] Map build completed!")
	if ssxl_engine:
		print("Final Tiles: %d" % ssxl_engine.get_current_tile_count())

# signal build_map_stopped() [signals.rs]
func build_map_stopped():
	print("🛑 [Signal] Map build stopped prematurely.")

# signal generation_error(error_message: GString) [signals.rs]
func generation_error(error_message: String):
	push_error("❌ [Signal] Generation Error: %s" % error_message)

# signal tick_complete(current_tick: u64) [signals.rs]
func tick_complete(current_tick: int):
	# Used for system-wide synchronization if needed.
	pass

# signal tile_flip_updated(tile_id: i32, flip_frame: i32) [signals.rs]
func tile_flip_updated(tile_id: int, flip_frame: int):
	# Animation update for a specific tile.
	pass

# signal animation_update(percent_done: f32, new_atlas_coords: Vector2i) [signals.rs]
func animation_update(percent_done: float, new_atlas_coords: Vector2i):
	# Global animation state update.
	pass

# signal animation_state_changed(enabled: bool) [signals.rs]
func animation_state_changed(enabled: bool):
	print("🏃 [Signal] Animation State Changed: %s" % ["Disabled", "Enabled"][int(enabled)])

# signal engine_status_updated(status_message: GString) [signals.rs]
func engine_status_updated(status_message: String):
	print("💬 [Signal] Engine Status: %s" % status_message)
