extends SSXLConductor
class_name SSXLConductorWrapper

func _ready():
	print("SSXLConductorWrapper: ready (Rust backend initialized).")
	print("In tree:", is_inside_tree())
	print("Tree:", get_tree())
	print("Path:", get_path())
