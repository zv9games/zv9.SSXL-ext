extends Node

@export var map_width: int = 64
@export var map_height: int = 64
@export var map_seed: int = 12345
@export var generator_id: String = "0"
@export var auto_start: bool = true

@onready var conductor = $"../SSXLConductor"
@onready var tilemap = $"../SSXLTileMap"

# ✅ Prevent infinite restart loops
var has_started := false

# ✅ Throttle progress output (Option 2)
var last_pct := -1


func _ready():
	print("\n=== SSXL DEMO CONTROLLER READY ===")

	if conductor == null:
		printerr("❌ SSXLConductor not found in scene tree")
		return
	if tilemap == null:
		printerr("❌ TileMap not found in scene tree")
		return

	# Connect signals
	_safe_connect(conductor, "conductor_ready", "_on_conductor_ready")
	_safe_connect(conductor, "generation_started", "_on_generation_started")
	_safe_connect(conductor, "generation_progress", "_on_generation_progress")
	_safe_connect(conductor, "chunk_rendered", "_on_chunk_rendered")
	_safe_connect(conductor, "generation_finished", "_on_generation_finished")
	_safe_connect(conductor, "generation_error", "_on_generation_error")
	_safe_connect(conductor, "debug_event", "_on_debug_event")
	_safe_connect(conductor, "ssxl_event", "_on_ssxl_event")

	# Initialize SSXL runtime shell
	if conductor.has_method("initialize_runtime_shell"):
		conductor.initialize_runtime_shell(self)
	else:
		printerr("❌ Conductor missing method: initialize_runtime_shell(signal_receiver)")
		return

	# Bind the tilemap
	if conductor.has_method("set_tilemap"):
		conductor.set_tilemap(tilemap)
	else:
		printerr("❌ Conductor missing method: set_tilemap(tilemap)")
		return

	# Set generator (STRING)
	if conductor.has_method("set_generator"):
		conductor.set_generator(generator_id)
	else:
		printerr("❌ Conductor missing method: set_generator(id)")
		return

	print("✅ SSXL demo wired. Waiting for conductor_ready…")


func _safe_connect(source: Object, signal_name: String, method_name: String):
	if source == null:
		printerr("❌ Cannot connect '%s' — source is null" % signal_name)
		return

	if source.has_signal(signal_name):
		var err = source.connect(signal_name, Callable(self, method_name))
		if err != OK:
			printerr("❌ Failed to connect signal '%s' → %s (err %s)" %
				[signal_name, method_name, err])
	else:
		printerr("⚠️ Signal '%s' not found on conductor" % signal_name)


func _on_conductor_ready():
	print("✅ Conductor ready")

	if auto_start and not has_started:
		has_started = true
		await get_tree().process_frame
		await get_tree().process_frame   # ✅ Two frames for safety
		_start_demo_generation_safe()


func _start_demo_generation_safe() -> void:
	if conductor == null:
		return

	if not conductor.has_method("get_status"):
		print("⚠️ Conductor has no get_status(); starting generation anyway")
		_start_demo_generation()
		return

	var status_before = conductor.get_status()
	print("ℹ️ Conductor status before start:", status_before)

	if status_before == "Busy":
		print("⏳ Conductor is Busy, deferring one frame before retrying...")
		await get_tree().process_frame
		var status_after = conductor.get_status()
		print("ℹ️ Conductor status after defer:", status_after)
		if status_after == "Busy":
			printerr("❌ Conductor still Busy after defer; aborting demo start to avoid ConductorBusy")
			return

	_start_demo_generation()


func _start_demo_generation():
	if conductor == null:
		return

	var config := {
		"width": map_width,
		"height": map_height,
		"seed": map_seed,
		"generator_id": generator_id,
		"demo": true,
	}

	print("\n=== SSXL DEMO: BUILD MAP REQUEST ===")
	print("Config:", config)

	if not conductor.has_method("build_map"):
		printerr("❌ Conductor missing method: build_map(config)")
		return
	if not conductor.has_method("start_generation"):
		printerr("❌ Conductor missing method: start_generation(target_tilemap)")
		return

	conductor.build_map(config)
	var ok: bool = conductor.start_generation(tilemap)

	if not ok:
		printerr("❌ start_generation() returned false (see Rust logs for ConductorBusy or other errors)")
		return

	print("🚀 SSXL generation started\n")


func _on_generation_started(tilemap_id: int, total_chunks: int):
	print("📡 generation_started → tilemap_id=%s, total_chunks=%s" %
		[tilemap_id, total_chunks])


# ✅ Throttled progress output (Option 2)
func _on_generation_progress(completed: int, total: int, metrics: Dictionary):
	if total <= 0:
		return

	var pct := int(float(completed) / float(total) * 100.0)

	if pct != last_pct:
		last_pct = pct
		print("📊 %s%% complete (%s/%s)" % [pct, completed, total])


# ✅ Silence chunk spam (optional)
func _on_chunk_rendered(completed: int, total: int):
	pass


func _on_generation_finished(tilemap_id: int):
	print("🏁 generation_finished → tilemap_id=%s" % tilemap_id)
	print("✅ SSXL DEMO: Map generation complete\n")


func _on_generation_error(message: String):
	printerr("❌ generation_error → %s" % message)


func _on_debug_event(message: String):
	print("🐞 debug_event →", message)


func _on_ssxl_event(event: Dictionary):
	print("📡 ssxl_event →", event)
