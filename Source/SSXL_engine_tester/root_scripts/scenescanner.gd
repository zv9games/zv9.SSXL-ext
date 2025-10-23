extends Node

signal tree_ready(map)

func scan_tree():
	var map = {}
	_recursive_scan(get_tree().root, map)
	emit_signal("tree_ready", map)

func _recursive_scan(node: Node, map: Dictionary):
	map[node.name.to_lower()] = node.get_path()
	for child in node.get_children():
		_recursive_scan(child, map)
