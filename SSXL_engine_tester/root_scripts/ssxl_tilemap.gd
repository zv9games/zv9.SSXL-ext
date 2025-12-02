# ssxl_tilemap.gd
extends TileMap

# This method is called by the Rust ChunkPresenter via a deferred FFI call.
func receive_chunk_data(binary_data: PackedByteArray) -> void:
	print("📢 SSXL_TILEMAP: Received chunk data (Size: %d bytes)" % binary_data.size()) # <-- DIAGNOSTIC 1
	
	# 1. Deserialize the PackedByteArray back into a Dictionary.
	var batch_data = bytes_to_var(binary_data)
	
	# CRITICAL FIX: Check if deserialization failed (returns Nil)
	if not batch_data is Dictionary:
		push_error("❌ FFI ERROR: Deserialization failed. Received Nil from bytes_to_var(). Check Rust serialization/var_to_bytes format.")
		return
	
	# 2. Extract the batched arrays from the Dictionary
	# IMPORTANT: The batch dictionary must contain these keys with valid Godot types
	var layer: int = batch_data.get("layer", 0)
	var positions: Array[Vector2i] = batch_data.get("positions", [])
	var source_ids: Array[int] = batch_data.get("source_ids", [])
	var atlas_coords: Array[Vector2i] = batch_data.get("atlas_coords", [])
	var alt_tiles: Array[int] = batch_data.get("alt_tiles", [])
	
	# DIAGNOSTIC 2
	print("📢 SSXL_TILEMAP: Deserialization successful. Placing %d tiles." % positions.size()) 
	
	if positions.is_empty():
		return

	# 3. Iterative set_cell() rendering
	# Note: This loop can be slow for 147k tiles.
	var tiles_set: int = 0
	for i in range(positions.size()):
		# We confirmed Source ID 0 is correct, but ensure Atlas Coords are valid.
		self.set_cell(
			layer,
			positions[i],
			source_ids[i],
			atlas_coords[i],
			alt_tiles[i]
		)
		tiles_set += 1

	print("✅ SSXL_TILEMAP: Successfully set %d tiles. Requesting update." % tiles_set)
	
	# 4. Request a manual TileMap update
	self.force_update()
