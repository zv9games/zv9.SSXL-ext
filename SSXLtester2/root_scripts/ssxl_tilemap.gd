extends TileMap

# ============================================================================
# 1. NODE REFERENCES
# ============================================================================

@onready var ssxl_signals = get_parent().get_node("SSXLSignals")
@onready var ssxl_engine = get_parent().get_node("SSXLEngine")


# ============================================================================
# 2. LIFECYCLE AND SIGNAL CONNECTION
# ============================================================================

func _ready():
	if ssxl_signals:
		# Connect to completion signals from the GDExtension core, broadcast 
		# through the SSXLSignals node.
		
		# signal build_map_complete() [signals.rs] - (Works with snake_case)
		ssxl_signals.connect("build_map_complete", Callable(self, "_on_build_map_complete"))
		
		# signal chunk_applied(key_x: i64, key_y: i64) [lib.rs]
		ssxl_signals.connect("chunk_applied", Callable(self, "_on_chunk_applied"))

		# Optional: Listen for batch updates if the engine sends data this way
		ssxl_signals.connect("chunk_generated_batch", Callable(self, "_on_chunk_generated_batch"))
		
		print("TileMap: Listening for SSXL generation signals.")
	else:
		push_error("SSXLSignals node not found. TileMap cannot track generation state.")


# ============================================================================
# 3. SIGNAL HANDLERS
# ============================================================================

# Handler for: signal build_map_complete()
func _on_build_map_complete():
	print("✅ [TileMap] Full map generation finished.")
	if ssxl_engine:
		print("Total Tiles set: %d" % ssxl_engine.get_current_tile_count())

# Handler for: signal chunk_applied(key_x: i64, key_y: i64)
func _on_chunk_applied(key_x: int, key_y: int):
	print("✨ [TileMap] Chunk data applied at key: (%d, %d)." % [key_x, key_y])
	
	# Force refresh so the Rust-written chunk is visible immediately
	self.force_update()

# Handler for: signal chunk_generated_batch(batch: Dictionary)
func _on_chunk_generated_batch(batch: Dictionary):
	print("📦 [TileMap] Received a batch of %d chunks for potential manual application." % batch.size())
