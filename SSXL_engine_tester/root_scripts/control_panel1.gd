extends Control

# 🧭 UI Node References
@onready var grid_width: SpinBox = $/root/main/ControlPanel/GridWidth
@onready var grid_height: SpinBox = $/root/main/ControlPanel/GridHeight
@onready var seed_input: LineEdit = $/root/main/ControlPanel/SeedInput
@onready var placement_mode_selector: OptionButton = $/root/main/ControlPanel/PlacementModeSelector
@onready var animate_checkbox: CheckBox = $/root/main/ControlPanel/AnimateCheckbox
@onready var status_label: Label = $/root/main/ControlPanel/StatusLabel
@onready var generate_button: Button = $/root/main/ControlPanel/GenerateButton
@onready var progress_bar: ProgressBar = $/root/main/ControlPanel/GenerationProgress
@onready var toggle_terminal_button: Button = $/root/main/ControlPanel/ToggleTerminalButton
@onready var clock_label: Label = $/root/main/TileMap/clocklabel
@onready var clock_timer: Timer = $/root/main/TileMap/clocktimer
@onready var controlpanel: Control = $/root/main/ControlPanel

# 🧠 Engine & Scene References
@onready var aetherion_engine: Node = get_node("/root/main/AetherionEngine")
@onready var aetherion_signals: Node = get_node("/root/main/AetherionSignals")
@onready var expansive_tilemap: TileMap = get_node("/root/main/Expansive_TileMap")
@onready var camera_tilemap: Camera2D = get_node("/root/main/Cameras/Camera2")

# 📊 State Variables
var last_percent: int = -1
var tile_size: Vector2 = Vector2(16, 16)
var panel_collapsed := false

# 🧭 Boot Sequence
func _ready() -> void:
	clock_timer.timeout.connect(_on_ClockTimer_timeout)
	_setup_ui()
	_connect_signals()

	var tileset := expansive_tilemap.get_tileset()
	if tileset:
		tile_size = tileset.get_tile_size()

# 🧩 UI Setup
func _setup_ui() -> void:
	grid_width.max_value = 1_000_000_000
	grid_height.max_value = 1_000_000_000
	grid_width.step = 1
	grid_height.step = 1
	grid_width.value = 10
	grid_height.value = 10

	placement_mode_selector.clear()
	placement_mode_selector.add_item("Noise")
	placement_mode_selector.add_item("Checkerboard")
	placement_mode_selector.add_item("Clustered")
	placement_mode_selector.select(0)

	progress_bar.min_value = 0
	progress_bar.max_value = 100
	progress_bar.value = 0
	progress_bar.visible = false

	status_label.text = "🟢 Ready to generate."

# 🔗 Signal Wiring
func _connect_signals() -> void:
	generate_button.pressed.connect(_on_generate_pressed)
	toggle_terminal_button.pressed.connect(_on_ToggleTerminalButton_pressed)

	if not aetherion_signals.build_map_start.is_connected(_on_build_map_start):
		aetherion_signals.build_map_start.connect(_on_build_map_start)
	if not aetherion_signals.map_building_status.is_connected(_on_map_building_status):
		aetherion_signals.map_building_status.connect(_on_map_building_status)
	if not aetherion_signals.generation_progress.is_connected(_on_generation_progress):
		aetherion_signals.generation_progress.connect(_on_generation_progress)
	if not aetherion_signals.generation_complete.is_connected(_on_generation_complete):
		aetherion_signals.generation_complete.connect(_on_generation_complete)

# 🔄 Frame Tick
func _process(_delta: float) -> void:
	aetherion_engine.call("aetherionoracle")

# 🚀 Generation Trigger
func _on_generate_pressed() -> void:
	var width := int(grid_width.value)
	var height := int(grid_height.value)
	var seed_text := seed_input.text
	var animate := animate_checkbox.button_pressed
	var mode := placement_mode_selector.get_item_text(placement_mode_selector.get_selected()).to_lower()

	if width <= 0 or height <= 0 or width * height > 1_000_000_000:
		status_label.text = "⚠️ Invalid grid size. Must be positive and total tiles must not exceed 1 billion."
		return

	var seed: int
	if seed_text.is_empty() or not seed_text.is_valid_int():
		seed = randi() % 1_000_000
		seed_input.text = str(seed)
		status_label.text = "⚠️ Invalid seed. Using random seed: %d" % seed
	else:
		seed = int(seed_text)

	var black := Vector2i(0, 0)
	var blue := Vector2i(1, 0)

	progress_bar.value = 0
	progress_bar.visible = true
	last_percent = -1
	status_label.text = "🧬 Generating map with mode: %s, animate: %s..." % [mode, str(animate)]
	generate_button.disabled = true

	expansive_tilemap.clear()
	camera_tilemap.enabled = true
	camera_tilemap.make_current()
	camera_tilemap.zoom = Vector2(1.0, 1.0)

	aetherion_engine.set_signals_node(aetherion_signals)
	aetherion_engine.set_tilemap(expansive_tilemap)
	aetherion_engine.build_custom_map(width, height, seed, mode, animate, black, blue)

# 📡 Signal Handlers
func _on_build_map_start() -> void:
	status_label.text = "🚀 Map generation started..."

func _on_map_building_status(status_message: String) -> void:
	status_label.text = "📢 %s" % status_message

func _on_generation_progress(percent: int) -> void:
	progress_bar.value = percent
	last_percent = percent
	expansive_tilemap.force_update(0)

func _on_generation_complete(results: Dictionary) -> void:
	progress_bar.visible = false
	generate_button.disabled = false

	var width = results.get("width", 0)
	var height = results.get("height", 0)
	var mode = results.get("mode", "unknown")
	var animate = results.get("animate", false)
	var duration = results.get("duration", 0.0)

	status_label.text = "✅ Map generation complete: %dx%d, mode: %s, animate: %s, duration: %.2fs" % [
		width, height, mode, str(animate), duration
	]

	camera_tilemap.global_position = Vector2(width * tile_size.x / 2, height * tile_size.y / 2)
	camera_tilemap.zoom = Vector2(1.0 / max(width / 10.0, 1.0), 1.0 / max(height / 10.0, 1.0))
	expansive_tilemap.force_update(0)

# 🕒 Clock Update
func _on_ClockTimer_timeout() -> void:
	clock_label.text = "🕒 " + Time.get_datetime_string_from_system()

# 🪄 Terminal Toggle
func _on_ToggleTerminalButton_pressed() -> void:
	panel_collapsed = !panel_collapsed

	for child in controlpanel.get_children():
		if child != toggle_terminal_button and child is Control:
			child.visible = not panel_collapsed

	clock_label.visible = not panel_collapsed
	progress_bar.visible = not panel_collapsed

	var main := get_node("/root/main") as Node
	main.call("_toggle_camera")
