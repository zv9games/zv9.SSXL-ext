extends Node

# Use the more specific Node type hint for GDExtension nodes
var engine_ref: Node = null
var status: String = "Unknown"

# Links this monitor to the SSXLEngine node and establishes signal communication.
func set_engine(engine: Node) -> void:
	# 1. Disconnect safely if a previous engine was linked
	if is_instance_valid(engine_ref) and engine_ref.has_signal("status_updated"):
		# Check if the signal is ALREADY connected before trying to disconnect
		if engine_ref.status_updated.is_connected(Callable(self, "_on_status_updated")):
			engine_ref.status_updated.disconnect(Callable(self, "_on_status_updated"))

	# 2. Set the new engine reference
	engine_ref = engine
	print("✅ EngineMonitor: Engine linked.")

	# 3. Connect the signal (now guaranteed to connect only once)
	if is_instance_valid(engine_ref) and engine_ref.has_signal("status_updated"):
		# Use the modern signal syntax for cleaner code
		engine_ref.status_updated.connect(_on_status_updated)
		print("📶 EngineMonitor: Connected to 'status_updated' signal.")
	else:
		push_warning("⚠️ EngineMonitor: Engine missing 'status_updated' signal.")


# Polls the engine for its current status (used by the engine_timer).
func update_status() -> void:
	if not is_instance_valid(engine_ref):
		push_error("🚨 EngineMonitor: Engine not linked.")
		return

	# Check for the existence of the method (essential for FFI/GDExtension calls)
	if not engine_ref.has_method("get_status"):
		push_error("🚨 EngineMonitor: Engine missing 'get_status' method.")
		return

	# Use call() for safety when interacting with GDExtension
	status = engine_ref.call("get_status") as String
	print("📡 EngineMonitor: Polled status → %s" % status)


# Handler for the status_updated signal (Asynchronous update from Rust).
func _on_status_updated(status_msg: String) -> void:
	status = status_msg
	print("📡 EngineMonitor: Received status → %s" % status)
