# main.gd — FINAL VERSION (2025 SSXL FFI Sync) — RENDERING FIXED
extends Node2D

signal ffi_core_ready

@onready var ssxl_oracle: Node = $"/root/ssxltester/SSXLOracle"
@onready var ssxl_engine: Node = $"/root/ssxltester/SSXLEngine"
@onready var ssxl_signals: Node = $"/root/ssxltester/SSXLSignals"
@onready var ssxl_chunk_node: Node = $"/root/ssxltester/SSXLChunk"
@onready var ssxl_tilemap: TileMap = $SSXLTilemap
@onready var clock_timer: Timer = $clocktimer
@onready var control_panel = $controlpanel

func _ready() -> void:
	print("\nSSXL Main: Quantum Boot Sequence Initiated")
	_establish_ffi_links()
	
	if not is_instance_valid(control_panel):
		push_error("FATAL: ControlPanel missing!")
		return
	
	_connect_clock_timer()
	
	# First tick to wake the oracle
	if is_instance_valid(ssxl_oracle):
		ssxl_oracle.tick()
		print("Oracle: First tick delivered.")

func _process(delta: float) -> void:
	# CRITICAL: Continuous tick loop to drain poller messages
	if is_instance_valid(ssxl_engine):
		ssxl_engine.tick(Time.get_ticks_msec())
		# Uncomment for heartbeat:
		# print("DEBUG: ssxl_engine.tick called")

func _establish_ffi_links() -> void:
	print("\nFFI LINK PHASE: Engaging Rust Core...")

	# 1. Validate critical FFI nodes
	if not (is_instance_valid(ssxl_oracle) and is_instance_valid(ssxl_engine)):
		push_error("FATAL: SSXLOracle or SSXLEngine missing from scene tree!")
		return

	# 2. Oracle → Engine
	ssxl_oracle.set_engine(ssxl_engine)
	print("Oracle → Engine linked")

	# 3. Presenter injection
	if not is_instance_valid(ssxl_chunk_node):
		push_error("FATAL: SSXLChunk presenter node missing!")
		return
	ssxl_engine.initialize_presenter(ssxl_chunk_node)
	print("Engine: Chunk presenter initialized")

	# 4. TileMap injection (assert the class at runtime)
	if not is_instance_valid(ssxl_tilemap):
		push_error("FATAL: SSXLTileMap missing!")
		return
	print("DEBUG: TileMap class =", ssxl_tilemap.get_class())
	ssxl_engine.set_tilemap(ssxl_tilemap)
	print("Engine: SSXLTileMap injected")

	# 5. Signals node (do this BEFORE Phase 1 so Rust can emit immediately)
	if is_instance_valid(ssxl_signals):
		ssxl_engine.set_signals_node(ssxl_signals)
		print("Engine → SSXLSignals linked")
	else:
		push_warning("SSXLSignals missing — no async callbacks!")
		return

	# 6. Phase 1: Runtime shell
	if not ssxl_engine.initialize_runtime_shell(""):
		push_error("FATAL: Runtime shell failed (Phase 1)")
		return
	print("Engine: Runtime shell ready")

	# 7. Phase 2: Spawn conductors
	if not ssxl_engine.start_async_conductors():
		push_error("FATAL: Conductors failed to spawn (Phase 2)")
		return
	print("Engine: Async conductors ALIVE")

	# 8. Connect critical signals safely (avoid duplicates)
	_connect_ffi_signals()

	print("FFI CORE FULLY INITIALIZED — EMITTING READY SIGNAL")
	call_deferred("emit_signal", "ffi_core_ready")

func _connect_ffi_signals() -> void:
	# chunk_data_ready → tile placement
	var cd_callable := Callable(control_panel.gen_logic, "_on_chunk_data_ready")
	if ssxl_signals.has_signal("chunk_data_ready"):
		if not ssxl_signals.is_connected("chunk_data_ready", cd_callable):
			ssxl_signals.chunk_data_ready.connect(cd_callable)
			print("chunk_data_ready → _on_chunk_data_ready CONNECTED — RENDERING ENABLED")
		else:
			print("chunk_data_ready → already connected, skipping")
	else:
		push_error("FATAL: 'chunk_data_ready' signal missing on SSXLSignals!")

	# Optional: hook start/complete/error for UI feedback
	var start_callable := Callable(control_panel.signal_handler, "_on_build_map_start")
	if ssxl_signals.has_signal("build_map_start") and not ssxl_signals.is_connected("build_map_start", start_callable):
		ssxl_signals.build_map_start.connect(start_callable)

	var complete_callable := Callable(control_panel.signal_handler, "_on_build_map_complete")
	if ssxl_signals.has_signal("build_map_complete") and not ssxl_signals.is_connected("build_map_complete", complete_callable):
		ssxl_signals.build_map_complete.connect(complete_callable)

	var err_callable := Callable(control_panel.signal_handler, "_on_generation_error")
	if ssxl_signals.has_signal("generation_error") and not ssxl_signals.is_connected("generation_error", err_callable):
		ssxl_signals.generation_error.connect(err_callable)

func _connect_clock_timer() -> void:
	if is_instance_valid(clock_timer) and is_instance_valid(control_panel.utility):
		clock_timer.timeout.connect(control_panel.utility.on_clock_timer_timeout)
		clock_timer.start()
		print("Clock timer connected")
	else:
		push_error("Clock timer setup failed!")

# Hook for root.gd / SSXLTester
func enter_idle_state() -> void:
	print("Main: Received control from root.gd — Entering idle state.")
