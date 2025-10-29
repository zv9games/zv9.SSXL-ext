@tool
extends EditorPlugin

var ssxl_instance: Node = null

func _enter_tree() -> void:
    print("🌀 SSXL plugin booting...")
    ProjectSettings.set_setting("ssxl/plugin_enabled", true)

    var script_path := "res://addons/S2O_godot_plugin/ssxl.gd"
    if ResourceLoader.exists(script_path):
        var script_res := load(script_path)
        if script_res and script_res is Script:
            ssxl_instance = script_res.new()
            if ssxl_instance:
                get_tree().root.add_child(ssxl_instance)
                print("✨ SSXL instance added to scene tree.")
                if ssxl_instance.has_method("enter_idle"):
                    ssxl_instance.call_deferred("enter_idle")
                else:
                    print("⚠️ SSXL instance missing 'enter_idle' method.")
            else:
                push_error("❌ Failed to instantiate SSXL script.")
        else:
            push_error("❌ Invalid script resource at: %s" % script_path)
    else:
        push_error("❌ SSXL script not found at: %s" % script_path)

func _exit_tree() -> void:
    print("🧹 SSXL plugin dismissed.")
    ProjectSettings.set_setting("ssxl/plugin_enabled", false)

    if is_instance_valid(ssxl_instance):
        ssxl_instance.queue_free()
        print("🧺 SSXL instance freed.")
    ssxl_instance = null
