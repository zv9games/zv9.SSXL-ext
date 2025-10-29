extends Node

func enter_idle():
    print("🌙 SSXL in idle mode.")
    call_deferred("_bind_idle")

func _bind_idle():
    if Engine.is_editor_hint():
        print("🧿 Editor context detected. Idle phase skipped.")
        return

    if has_method("start_idle"):
        start_idle()
    else:
        print("⚠️ No idle method found. Skipping ticker.")

func start_idle():
    print("🪶 Idle ticker started.")
    # Optional: set up heartbeat, overlay, or runtime polling

func activate():
    print("⚡ SSXL activated.")
    # Optional: call_deferred("_bind_active")

func teardown():
    print("🧺 SSXL teardown.")
    # Optional: stop ticker, release ligatures
