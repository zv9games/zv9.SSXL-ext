extends Node

func _ready():
	pass


func dump_tree(node: Node, indent := 0):
	print("  ".repeat(indent) + str(node))
	for child in node.get_children():
		dump_tree(child, indent + 1)
