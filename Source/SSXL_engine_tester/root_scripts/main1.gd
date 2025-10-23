extends Node2D

# 🧭 Node references
@onready var control_panel: Control = get_node("/root/main/ControlPanel")
@onready var aetherion_engine: Node = get_node("/root/main/AetherionEngine")
@onready var aetherion_signals: Node = get_node("/root/main/AetherionSignals")
@onready var expansive_tilemap: TileMap = get_node("/root/main/Expansive_TileMap")
@onready var camera1: Camera2D = get_node("/root/main/Cameras/Camera1")
@onready var camera2: Camera2D = get_node("/root/main/Cameras/Camera2")
@onready var terminal_output: RichTextLabel = get_node_or_null("/root/main/ControlPanel/TerminalOutput")


# 📊 State
var dragging: bool = false
var active_camera: int = 1
var last_mouse_position: Vector2 = Vector2.ZERO
var tile_size: Vector2 = Vector2(16, 16)

func _ready() -> void:
	if not aetherion_signals.generation_complete.is_connected(_on_generation_complete):
		aetherion_signals.generation_complete.connect(_on_generation_complete)

	# 🛡️ Prevent duplicate Main nodes
	var main_nodes := get_tree().get_nodes_in_group("Main")
	if main_nodes.size() > 1:
		GlobalLogger.log("⚠️ Multiple Main nodes detected: %s" % main_nodes, "WARN")
		queue_free()
		return

	
	GlobalLogger.log("🟢 Main.gd is ready. AetherionEngine is idle.")
	GlobalLogger.log("⚠️ Multiple Main nodes detected: %s" % [main_nodes], "WARN")


	# 🎥 Camera configuration
	camera1.enabled = true
	camera1.make_current()
	camera2.enabled = false
	camera2.zoom = Vector2(1.0, 1.0)

	# 🔧 Initialize AetherionEngine
	aetherion_engine.set_signals_node(aetherion_signals)
	aetherion_engine.set_tilemap(expansive_tilemap)

	# 🧪 Debug: TileMap state
	GlobalLogger.log("TileMap position: %s" % expansive_tilemap.global_position)
	GlobalLogger.log("TileMap scale: %s" % expansive_tilemap.scale)
	GlobalLogger.log("TileMap layer enabled: %s" % expansive_tilemap.is_layer_enabled(0))
	GlobalLogger.log("TileMap tileset: %s" % expansive_tilemap.get_tileset())
	GlobalLogger.log("TileMap used rect: %s" % expansive_tilemap.get_used_rect())

	# 🧪 Debug: Camera states
	GlobalLogger.log("Camera1 position: %s" % camera1.global_position)
	GlobalLogger.log("Camera1 enabled: %s" % camera1.enabled)
	GlobalLogger.log("Camera1 is current: %s" % camera1.is_current())
	GlobalLogger.log("Camera2 position: %s" % camera2.global_position)
	GlobalLogger.log("Camera2 zoom: %s" % camera2.zoom)
	GlobalLogger.log("Camera2 enabled: %s" % camera2.enabled)
	GlobalLogger.log("Camera2 is current: %s" % camera2.is_current())

	# 🧠 TileSet analysis
	var tileset := expansive_tilemap.get_tileset()
	if tileset:
		tile_size = tileset.get_tile_size()
		GlobalLogger.log("TileSet tile size: %s" % tile_size)

		if tileset.get_source_count() > 0:
			var source := tileset.get_source(0)
			if source and source.is_class("TileSetAtlasSource"):
				var atlas_source := source as TileSetAtlasSource
				GlobalLogger.log("TileSet atlas grid size: %s" % atlas_source.get_atlas_grid_size())
				GlobalLogger.log("Tile (0, 0) exists: %s" % atlas_source.has_tile(Vector2i(0, 0)))
				GlobalLogger.log("Tile (1, 0) exists: %s" % atlas_source.has_tile(Vector2i(1, 0)))

func _toggle_camera() -> void:
	var status_label: Label = get_node("/root/main/ControlPanel/StatusLabel")

	if active_camera == 1:
		if camera2.is_inside_tree() and camera2.enabled:
			camera2.make_current()
			active_camera = 2
			status_label.text = "🧭 Switched to Map View"
			GlobalLogger.log("🎥 Camera2 is now active.")
		else:
			GlobalLogger.log("⚠️ Camera2 is not ready.", "WARN")
	else:
		if camera1.is_inside_tree() and camera1.enabled:
			camera1.make_current()
			active_camera = 1
			status_label.text = "🎛️ Returned to Control Panel"
			GlobalLogger.log("🎥 Camera1 is now active.")
		else:
			GlobalLogger.log("⚠️ Camera1 is not ready.", "WARN")

func _unhandled_input(event: InputEvent) -> void:
	#GlobalLogger.log("Active camera: %d | Zoom: %s" % [active_camera, camera2.zoom])

	if event is InputEventKey and event.pressed and event.keycode == KEY_SPACE:
		_toggle_camera()

	if active_camera == 2:
		if event is InputEventMouseButton:
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				camera2.zoom *= 0.9
			elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				camera2.zoom *= 1.1

			camera2.zoom.x = clamp(camera2.zoom.x, 0.5, 5.0)
			camera2.zoom.y = clamp(camera2.zoom.y, 0.5, 5.0)

			if event.button_index == MOUSE_BUTTON_LEFT:
				dragging = event.pressed
				last_mouse_position = event.position

		if event is InputEventMouseMotion and dragging:
			var delta: Vector2 = event.position - last_mouse_position
			camera2.position -= delta / camera2.zoom
			last_mouse_position = event.position

func _on_generation_complete(results: Dictionary) -> void:
	GlobalLogger.log("🧭 Generation complete signal received.")
	_toggle_camera()
