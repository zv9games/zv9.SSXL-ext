extends SceneTree

var received_event := false
var received_payload: Dictionary

func _init():
    print("\n=== SSXL HEADLESS TEST RUNNER (TANK MODE) ===")

    var c := SSXLConductor.new()
    if c == null:
        printerr("❌ FAILED: SSXLConductor could not be instantiated")
        self.quit(1)
        return

    var signals := c.get_signal_list()
    var names := signals.map(func(s): return s.name)

    if "ssxl_event" not in names:
        printerr("❌ FAILED: Missing signal 'ssxl_event'")
        self.quit(1)
        return
    print("✅ Signal exists: ssxl_event")

    var methods := c.get_method_list()
    var method_names := methods.map(func(m): return m.name)

    if "test_emit_event" not in method_names:
        printerr("❌ FAILED: Missing method 'test_emit_event'")
        self.quit(1)
        return
    print("✅ Method exists: test_emit_event")

    var err := c.connect("ssxl_event", Callable(self, "_on_event"))
    if err != OK:
        printerr("❌ FAILED: Could not connect to ssxl_event (error %s)" % err)
        self.quit(1)
        return
    print("✅ Connected to ssxl_event")

    print("→ Calling c.test_emit_event()")
    c.test_emit_event()

    await self.process_frame

    if not received_event:
        printerr("❌ FAILED: ssxl_event did not fire")
        self.quit(1)
        return

    if typeof(received_payload) != TYPE_DICTIONARY:
        printerr("❌ FAILED: ssxl_event payload was not a Dictionary")
        self.quit(1)
        return

    if not received_payload.has("type") or received_payload.type != "rust_test_event":
        printerr("❌ FAILED: ssxl_event payload missing or incorrect 'type'")
        self.quit(1)
        return

    if not received_payload.has("ok") or received_payload.ok != true:
        printerr("❌ FAILED: ssxl_event payload missing or incorrect 'ok'")
        self.quit(1)
        return

    print("✅ Payload validated:", received_payload)

    print("\n✅✅✅ SSXL HEADLESS TESTS PASSED (TANK MODE) ✅✅✅\n")
    self.quit(0)


func _on_event(event):
    received_event = true
    received_payload = event
    print("📡 ssxl_event received:", event)
