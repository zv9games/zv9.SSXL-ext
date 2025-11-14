# control_panel.gd
extends Control

# ------------------------------------------------------------------------------
# --- CHUNK & RENDER CONSTANTS ⚙️ ---
# ------------------------------------------------------------------------------
# These constants define the dimensions used by the Rust generation logic.
const CHUNK_SIZE_X: int = 32
const CHUNK_SIZE_Y: int = 32
const DEFAULT_CHUNKS: int = 12
const BASE_SOURCE_ID: int = 0
const BASE_ATLAS_COORDS: Vector2i = Vector2i(0, 0)
const BASE_ALT_ID: int = 0

# ------------------------------------------------------------------------------
# 🧭 UI Node & External References (DATA HUB)
# ------------------------------------------------------------------------------
# All @onready references use explicit type hints for performance and clarity.
# UI Elements
@onready var grid_width: SpinBox = $gridwidthspinbox
@onready var grid_height: SpinBox = $gridheightspinbox
@onready var seed_input: LineEdit = $seedlineedit
@onready var placement_mode_selector: OptionButton = $placementoptionbutton
@onready var tile_type_selector: OptionButton = $tiletypeoptionbutton
@onready var animate_checkbox: CheckBox = $animatecheckbox
@onready var status_label: Label = $billboard
@onready var generate_button: Button = $map_gen_button
@onready var animate_ui_button: Button = $animate_button
@onready var progress_bar: ProgressBar = $progressbar
@onready var toggle_terminal_button: Button = $toggleterminalbutton

# Timers
@onready var engine_timer: Timer = $enginetimer             # Fast poll when NOT animating
@onready var animation_timer: Timer = $AnimationTimer       # Animation loop when animating
@onready var redraw_throttle_timer: Timer = $redrawthrottle # Map Gen batch redraw

# Timer Labels
@onready var engine_timer_label: Label = $enginetimerlabel
@onready var tiles_placed_label: Label = $tilesplacedlabel
@onready var tile_placement_time_label: Label = $tiletimeofplacement

# Scene Tree Nodes (Critical Engine References)
@onready var main: Node2D = get_parent() as Node2D
# Use NodePath strings directly in @onready where possible for robust reference initialization
@onready var ssxl_engine: Node = main.get_node("SSXLEngine")
@onready var ssxl_oracle: Node = main.get_node("SSXLOracle")
@onready var ssxl_signals: Node = main.get_node("SSXLSignals") # <-- CRITICAL FIX: Named correctly!
@onready var expansive_tilemap: TileMap = main.get_node("expansive_tilemap") as TileMap
@onready var clock_label: Label = main.get_node("tilemap/clocklabel") as Label
@onready var clock_timer: Timer = main.get_node("tilemap/clocktimer") as Timer
@onready var tilemap_node: Node2D = main.get_node("tilemap") as Node2D
@onready var cameras: Node = main.get_node("cameras")
@onready var camera1: Camera2D = main.get_node("cameras/camera1") as Camera2D
@onready var camera2: Camera2D = main.get_node("cameras/camera2") as Camera2D

# 📊 State Variables (Shared data)
var last_percent: int = -1
var tile_size: Vector2 = Vector2(16, 16)
var panel_collapsed: bool = false
var engine_tick_count: int = 0
var total_tiles_placed: int = 0
var is_generating: bool = false
var initial_zoom_set: bool = false
var current_camera_id: int = 1
var generation_start_time_ms: int = 0
var redraw_pending: bool = false
var is_animated: bool = false
var animation_tilemap_handle: TileMap = null # Initialize to null with explicit type

# 🔗 Composed Helper Scripts (Typed instantiation)
# Note: Type hints use the script resource's type for better safety
var ui_setup: control_panel_ui_setup
var gen_logic: control_panel_gen_logic
var signal_handler: control_panel_signal_handler
var animation_logic: control_panel_animation_logic
var utility: control_panel_utility


# ------------------------------------------------------------------------------
# 🧭 CORE LIFECYCLE (COORDINATION)
# ------------------------------------------------------------------------------

func _ready() -> void:
	# Use preload() for optimal, faster resource loading and type safety
	const SCRIPT_PATH = "res://root_scripts/"
	
	# 1. Instantiate Helper Scripts
	# Using preload().new() is generally faster than load().new()
	ui_setup = preload(SCRIPT_PATH + "controlpaneluisetup.gd").new(self) as control_panel_ui_setup
	gen_logic = preload(SCRIPT_PATH + "controlpanelgenlogic.gd").new(self) as control_panel_gen_logic
	signal_handler = preload(SCRIPT_PATH + "controlpanelsignalhandler.gd").new(self) as control_panel_signal_handler
	animation_logic = preload(SCRIPT_PATH + "controlpanelanimation.gd").new(self) as control_panel_animation_logic
	utility = preload(SCRIPT_PATH + "controlpanelutility.gd").new(self) as control_panel_utility

	# 2. Critical Node Validation (Ensure engine and map are present)
	if not is_instance_valid(expansive_tilemap):
		push_error("❌ FATAL: Initialization Error: TileMap missing.")
		if is_instance_valid(status_label):
			status_label.text = "❌ FATAL: TileMap missing."
		return

	# Validate Engine references (Crucial for GDExtension FFI)
	if not is_instance_valid(ssxl_engine) or not is_instance_valid(ssxl_signals):
		push_error("❌ FATAL: SSXLEngine or SSXLSignals GDExtension nodes missing.")
		return

	# --- 3. Delegate Initialization Tasks ---
	ui_setup.setup_all()
	
	# Final ready steps
	if is_instance_valid(cameras) and cameras.has_method("switch_to_camera"):
		cameras.switch_to_camera(current_camera_id)

	var tileset: TileSet = expansive_tilemap.get_tileset()
	if tileset:
		tile_size = tileset.get_tile_size()

	# Initialize animation state
	if is_instance_valid(animate_checkbox):
		animation_logic.on_animate_checkbox_toggled(animate_checkbox.button_pressed)

	# --- 4. Connect Engine Signals to Handlers ---
	
	# High-Frequency Signals (Animation)
	# This signal is only connected here and in ui_setup.gd (for animation checkbox)
	# The animation logic relies on the GDExtension's ability to emit high-frequency data.
	ssxl_signals.tile_flip_updated.connect(animation_logic._on_tile_flip_updated) 
	
	# Status signal: check existence before connecting (Good practice preserved)
	if ssxl_signals.has_signal("engine_status_updated"):
		ssxl_signals.engine_status_updated.connect(signal_handler._on_engine_status_updated)
	else:
		push_error("❌ FATAL: Engine status signal not found. Check Rust GDExtension.")


	# --- 5. Connect Timers to Logic (Separating concerns) ---
	
	# Map Generation Redraw Throttling (Map Gen Draw)
	redraw_throttle_timer.timeout.connect(utility._on_redraw_throttle_timeout)

	# Animation/Polling Timers (Animation Draw & Status Polling)
	animation_timer.timeout.connect(animation_logic._on_animation_timer_timeout)
	engine_timer.timeout.connect(animation_logic._on_engine_timer_timeout)
	
	# Ensure one polling mechanism is active
	animation_logic.setup_animation_worker(is_animated)


func _input(event: InputEvent) -> void:
	utility.handle_input_event(event)


func _process(_delta: float) -> void:
	# Process polling is handled by timers. This remains empty, prioritizing
	# asynchronous Rust conduction over synchronous _process() polling.
	pass
