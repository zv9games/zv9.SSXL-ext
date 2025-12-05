# tilemap.gd
extends TileMap
class_name SSXLTilemap

const TILESET_ID := 1
const CHUNK_SIZE := 32
const TILE_LAYER := 0

var loaded_chunks := {}

func _ready():
	# Connect to the FFI response
	get_node("/root/SSXLSignals").chunk_loaded.connect(self._on_chunk_loaded)

# 🧱 O(N) application of tile data (N = tiles in chunk)
func _on_chunk_loaded(key: Vector2i, data: Dictionary):
	if loaded_chunks.has(key): return

	loaded_chunks[key] = true
	var map_origin = key * CHUNK_SIZE
	var tiles: Array = data.tiles
	
	# Batch update loop
	for tile in tiles:
		var cell_pos = Vector2i(map_origin.x + tile.local_x, map_origin.y + tile.local_y)
		var tile_id = tile.id
		
		# O(1) TileMap call (repeated N times)
		set_cell(TILE_LAYER, cell_pos, TILESET_ID, Vector2i(tile_id, 0))
