extends SceneTree

var received_event := false
var received_payload: Dictionary

func _init():
    print("\n=== SSXL HEADLESS TEST RUNNER (TANK MODE) ===")
    
    var c := SSXLConductor.new()
    if c == null:
        printerr("❌ FAILED: SSXLConductor could not be instantiated")
        quit(1)
        return

    dump_full_ssxl_api()

    var signals_list = c.get_signal_list()
    var names = signals_list.map(func(s): return s["name"])
    if not "ssxl_event" in names:
        printerr("❌ FAILED: Missing signal 'ssxl_event'")
        quit(1)
        return

    var methods_list = c.get_method_list()
    var m_names = methods_list.map(func(m): return m["name"])
    if not "test_emit_event" in m_names:
        printerr("❌ FAILED: Missing method 'test_emit_event'")
        quit(1)
        return

    c.connect("ssxl_event", Callable(self, "_on_event"))
    c.test_emit_event()

    # Fixed: Since we extend SceneTree, we await our own process_frame
    await self.process_frame

    if not received_event:
        printerr("❌ FAILED: ssxl_event did not fire")
        quit(1)
        return

    print("✅ Payload validated:", received_payload)
    print("\n✅✅✅ SSXL HEADLESS TESTS PASSED (TANK MODE) ✅✅✅\n")
    quit(0)

func _on_event(event):
    received_event = true
    received_payload = event
    print("📡 ssxl_event received:", event)

func dump_full_ssxl_api():
    print("\n=== SSXL-EXT FULL API (AUTO-DISCOVERED) ===")
    var classes = ClassDB.get_class_list()
    classes.sort()
    for cls in classes:
        if cls.begins_with("SSXL"):
            print_api_for_class(cls)

func print_api_for_class(cls_name: String):
    print("\n--- CLASS: ", cls_name)
    var sigs = ClassDB.class_get_signal_list(cls_name)
    for s in sigs:
        print("  - [Signal] ", s["name"])
    var meths = ClassDB.class_get_method_list(cls_name)
    for m in meths:
        print("  - [Method] ", m["name"])