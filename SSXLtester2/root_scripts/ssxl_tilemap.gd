# SSXLTilemap.gd (Attach this to your SSXLTilemap node)
extends TileMap

# --- Configuration Export Variables ---

# MODIFIED: Maximum range set to 10000
@export_range(32, 10000, 64)
var map_width: int = 256
# MODIFIED: Maximum range set to 10000
@export_range(32, 10000, 64)
var map_height: int = 256

@export_enum("perlin_basic_2d:0", "other_generator:1")
var generator_name: String = "perlin_basic_2d"

# Use an arbitrary string for the seed
@export
var map_seed: String = "42_island_demo" 

# The "pattern" setting: This controls the scale of the Perlin noise in Rust.
# A larger number results in smoother, larger islands.
@export_range(50.0, 500.0, 10.0)
var perlin_scale: float = 128.0

# NEW: Tile Overrides Dictionary (coordinate Vector2i -> tile_id int)
@export var tile_overrides: Dictionary = {}


# --- Function to retrieve all settings as a Dictionary ---

func get_generation_config() -> Dictionary:
	return {
		"width": map_width,
		"height": map_height,
		"seed": map_seed,
		"generator": generator_name,
		"scale": perlin_scale,
		"tile_overrides": tile_overrides # <--- ADDED: Pass the overrides dictionary
	}
