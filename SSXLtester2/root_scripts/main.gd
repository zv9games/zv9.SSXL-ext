extends Node

# ============================================================================
# 1. NODE REFERENCES
# ============================================================================
@onready var ssxl_tilemap: TileMap = $SSXLTilemap
@onready var ssxl_oracle = $SSXLOracle
@onready var ssxl_engine = $SSXLEngine
@onready var ssxl_signals = $SSXLSignals

# ============================================================================
# 2. API CONFIGURATION (Editor‑exposed exports)
# ============================================================================
@export var map_width: int = 512
@export var map_height: int = 512
@export_enum("perlin", "ca") var generator_id: String = "perlin"
@export var base_tile_id: int = 0
@export var atlas_coord: Vector2i = Vector2i(0, 0)

# ============================================================================
# 3. LIFECYCLE MANAGEMENT
# ============================================================================
func _ready():
	print("Engine class: ", ssxl_engine.get_class())
	print("Oracle class: ", ssxl_oracle.get_class())
	print("Signals class: ", ssxl_signals.get_class())

	print("--- ⚙️ Initializing SSXL Runtime ---")

	ssxl_engine.call("initialize_runtime_shell", ssxl_signals)
	ssxl_oracle.set_engine(ssxl_engine)
	ssxl_engine.call("set_tilemap", ssxl_tilemap)

	ssxl_engine.call("set_generator", generator_id)

	# Build config dictionary for GDExtension build_map
	var config = {
		"width": map_width,
		"height": map_height,
		"generator_id": generator_id,
		"base_tile_id": base_tile_id,
		"atlas_coord": atlas_coord
	}

	var build_ok = ssxl_engine.call("build_map", config)
	print("Build map returned: %s" % str(build_ok))

	var status = ssxl_engine.get_status()
	var active_id = ssxl_engine.get_active_generator_id()
	print("Status: %s | Active generator: %s" % [status, active_id])

func _process(_delta):
	ssxl_oracle.tick()
