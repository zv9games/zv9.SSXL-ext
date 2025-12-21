extends Node

var last_indent := -1   # ✅ throttle repeated indent levels

func _ready():
	pass


func dump_tree(node: Node, indent := 0):
	# ✅ Only print when indent changes (Option 2 throttling)
	if indent != last_indent:
		last_indent = indent
		print("  ".repeat(indent) + str(node))

	for child in node.get_children():
		dump_tree(child, indent + 1)
