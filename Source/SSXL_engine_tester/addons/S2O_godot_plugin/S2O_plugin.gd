@tool
extends EditorPlugin

var aetherion_instance: Node = null

func _enter_tree() -> void:
    print("🌀 Aetherion plugin booting...")
    ProjectSettings.set_setting("aetherion/plugin_enabled", true)

    var script_path := "res://addons/S2O_godot_plugin/Aetherion.gd"
    if ResourceLoader.exists(script_path):
        var script_res := load(script_path)
        if script_res and script_res is Script:
            aetherion_instance = script_res.new()
            if aetherion_instance:
                get_tree().root.add_child(aetherion_instance)
                print("✨ Aetherion instance added to scene tree.")
                if aetherion_instance.has_method("enter_idle"):
                    aetherion_instance.call_deferred("enter_idle")
                else:
                    print("⚠️ Aetherion instance missing 'enter_idle' method.")
            else:
                push_error("❌ Failed to instantiate Aetherion script.")
        else:
            push_error("❌ Invalid script resource at: %s" % script_path)
    else:
        push_error("❌ Aetherion script not found at: %s" % script_path)

func _exit_tree() -> void:
    print("🧹 Aetherion plugin dismissed.")
    ProjectSettings.set_setting("aetherion/plugin_enabled", false)

    if is_instance_valid(aetherion_instance):
        aetherion_instance.queue_free()
        print("🧺 Aetherion instance freed.")
    aetherion_instance = null
