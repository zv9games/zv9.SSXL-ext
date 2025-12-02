extends Control

const CHUNK_SIZE_X: int = 32
const CHUNK_SIZE_Y: int = 32
const DEFAULT_CHUNKS: int = 12
const BASE_SOURCE_ID: int = 0
const BASE_ATLAS_COORDS: Vector2i = Vector2i(0, 0)
const BASE_ALT_ID: int = 0

# UI
@onready var grid_width: SpinBox = $gridwidthspinbox
@onready var grid_height: SpinBox = $gridheightspinbox
@onready var seed_input: LineEdit = $seedlineedit
@onready var placement_mode_selector: OptionButton = $placementoptionbutton
@onready var tile_type_selector: OptionButton = $tiletypeoptionbutton
@onready var status_label: Label = $billboard
@onready var generate_button: Button = $map_gen_button
@onready var animate_ui_button: Button = $animate_button
@onready var progress_bar: ProgressBar = $progressbar
@onready var toggle_terminal_button: Button = $toggleterminalbutton
@onready var engine_timer: Timer = $enginetimer
@onready var animation_timer: Timer = $AnimationTimer
@onready var redraw_throttle_timer: Timer = $redrawthrottle
@onready var engine_timer_label: Label = $enginetimerlabel
@onready var tiles_placed_label: Label = $tilesplacedlabel
@onready var tile_placement_time_label: Label = $tiletimeofplacement

# Scene hierarchy
@onready var main: Node2D = get_parent() as Node2D
@onready var cameras: Node = main.get_node("cameras")
@onready var clock_label: Label = main.get_node("clocklabel") as Label

# RUST FFI NODES
@onready var ssxl_engine: Node = get_node("/root/ssxltester/SSXLEngine")
@onready var ssxl_oracle: Node = get_node("/root/ssxltester/SSXLOracle")
@onready var ssxl_signals: Node = get_node("/root/ssxltester/SSXLSignals")

# CRITICAL: These are the ones used by gen_logic
@onready var ssxl_tilemap: Node = get_node("/root/ssxltester/main/SSXLTilemap")   # ← Rust tilemap
@onready var presenter: Node = get_node("/root/ssxltester/SSXLChunk")            # ← Chunk data holder

# Old reference — we keep it only for compatibility with some old code (safe to ignore)
@onready var expansive_tilemap: TileMap = main.get_node("SSXLTilemap") as TileMap

# State
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
var animation_tilemap_handle: TileMap = null

# Subsystems
var ui_setup
var gen_logic
var signal_handler
var animation_logic
var utility

func _ready() -> void:
	const SCRIPT_PATH: String = "res://root_scripts/"
	utility = preload(SCRIPT_PATH + "controlpanelutility.gd").new(self)
	ui_setup = preload(SCRIPT_PATH + "controlpaneluisetup.gd").new(self)
	gen_logic = preload(SCRIPT_PATH + "controlpanelgenlogic.gd").new(self)
	signal_handler = preload(SCRIPT_PATH + "controlpanelsignalhandler.gd").new(self)
	animation_logic = preload(SCRIPT_PATH + "controlpanelanimation.gd").new(self)

	# Basic validation
	if not is_instance_valid(ssxl_engine) or not is_instance_valid(ssxl_signals) or not is_instance_valid(ssxl_oracle):
		push_error("FATAL: Missing SSXL FFI nodes!")
		return

	if not is_instance_valid(ssxl_tilemap):
		push_error("FATAL: SSXLTileMap not found! Check scene path.")
		return

	if not is_instance_valid(presenter):
		push_error("FATAL: SSXLChunk (presenter) not found!")
		return

	ui_setup.setup_all()
	connect_all_timer_timeouts()

	animation_logic.setup_animation_worker(false)

	if is_instance_valid(cameras) and cameras.has_method("switch_to_camera"):
		cameras.switch_to_camera(current_camera_id)

	if main and main.has_signal("ffi_core_ready"):
		main.ffi_core_ready.connect(_on_ffi_core_ready, CONNECT_ONE_SHOT)

	print("Control Panel: Initialized. Awaiting FFI Core Alignment...")

func _on_ffi_core_ready() -> void:
	print("Control Panel: FFI Core Ready. Finalizing connections...")
	ui_setup.link_ffi_nodes()
	connect_all_ffi_signals()
	signal_handler._on_ffi_alignment_complete()

func connect_all_ffi_signals() -> void:
	print("connecting ffi signals...")

	if ssxl_signals.has_signal("tile_flip_updated"):
		if not ssxl_signals.is_connected("tile_flip_updated", Callable(animation_logic, "_on_tile_flip_updated")):
			ssxl_signals.tile_flip_updated.connect(Callable(animation_logic, "_on_tile_flip_updated"))

	if ssxl_signals.has_signal("engine_status_updated"):
		if not ssxl_signals.is_connected("engine_status_updated", Callable(signal_handler, "_on_engine_status_updated")):
			ssxl_signals.engine_status_updated.connect(Callable(signal_handler, "_on_engine_status_updated"))

	# CRITICAL FIX — only connect if NOT already connected
	if ssxl_signals.has_signal("chunk_data_ready"):
		if not ssxl_signals.is_connected("chunk_data_ready", Callable(gen_logic, "_on_chunk_data_ready")):
			ssxl_signals.chunk_data_ready.connect(Callable(gen_logic, "_on_chunk_data_ready"))
			print("chunk_data_ready → CONNECTED (first time)")
		else:
			print("chunk_data_ready → already connected, skipping")
	else:
		push_error("FATAL: 'chunk_data_ready' signal missing on SSXLSignals!")

	if ssxl_signals.has_signal("build_map_complete"):
		if not ssxl_signals.is_connected("build_map_complete", Callable(signal_handler, "_on_build_map_complete")):
			ssxl_signals.build_map_complete.connect(Callable(signal_handler, "_on_build_map_complete"))

func connect_all_timer_timeouts() -> void:
	redraw_throttle_timer.timeout.connect(utility._on_redraw_throttle_timeout)
	animation_timer.timeout.connect(animation_logic._on_animation_timer_timeout)

func _input(event: InputEvent) -> void:
	utility.handle_input_event(event)
