extends Node

# --- Signals ---
signal generation_started
signal generation_finished
signal metrics_ready(metrics: Dictionary)

# --- User-facing exports ---
@export var width_chunks: int = 4
@export var height_chunks: int = 4
@export var chunk_size: int = 32
@export var generator_id: String = "default"
@export var override_tile_id: int = -1

# --- Internal references ---
@onready var core: SSXLConductor = $"../SSXLConductor"
@onready var tilemap: TileMap = $"../TileMap"

func _ready():
	# Give the conductor its TileMap
	core.set_tilemap(tilemap)

	# Listen for Rust telling us “map is built”
	core.connect("map_built", Callable(self, "_on_map_built"))

	# Start the process next frame
	call_deferred("generate")


func generate():
	var config := {
		"width_chunks": width_chunks,
		"height_chunks": height_chunks,
		"chunk_size": chunk_size,
		"generator": generator_id,
		"override_tile": override_tile_id
	}

	emit_signal("generation_started")

	# ✅ Only build the map here
	# ❌ Do NOT start generation yet
	core.build_map(config)


func _on_map_built():
	# ✅ Rust says “map is ready”
	# ✅ Now it is safe to start generation
	core.start_generation(self)
