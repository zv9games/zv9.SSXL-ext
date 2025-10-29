extends AetherionSignals

signal ignition_fired(seed)
signal tilemap_ready(data)
signal engine_initialized
signal rust_event_received(payload)

func emit_ignition(seed):
	emit_signal("ignition_fired", seed)

func emit_tilemap_ready(data):
	emit_signal("tilemap_ready", data)

func emit_engine_initialized():
	emit_signal("engine_initialized")

func emit_rust_event(payload):
	emit_signal("rust_event_received", payload)
