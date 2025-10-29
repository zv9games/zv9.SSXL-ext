extends Node

var engine_ref: Object = null
var status: String = "Unknown"

func set_engine(engine: Object) -> void:
	engine_ref = engine
	print("✅ EngineMonitor: Engine linked.")

	if engine_ref.has_signal("status_updated"):
		engine_ref.connect("status_updated", Callable(self, "_on_status_updated"))
		print("📶 EngineMonitor: Connected to 'status_updated' signal.")
	else:
		push_warning("⚠️ EngineMonitor: Engine missing 'status_updated' signal.")

func update_status() -> void:
	if engine_ref == null:
		push_error("🚨 EngineMonitor: Engine not linked.")
		return

	if not engine_ref.has_method("get_status"):
		push_error("🚨 EngineMonitor: Engine missing 'get_status' method.")
		return

	status = engine_ref.call("get_status")
	print("📡 EngineMonitor: Polled status → %s" % status)

func _on_status_updated(status_msg: String) -> void:
	status = status_msg
	print("📡 EngineMonitor: Received status → %s" % status)
