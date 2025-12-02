extends Node

# Signal emitted when the scan is complete. It passes a Dictionary (map) 
# where keys are node names (lowercase) and values are their full NodePaths.
signal tree_ready(map: Dictionary)

# ----------------------------------------------------------------------------
# CORE FUNCTION: Initiates the scene tree scan and emits the completion signal.
# ----------------------------------------------------------------------------
func scan_tree():
	# The dictionary that will store the node name -> path mapping.
	var map: Dictionary = {}
	
	# Start the recursive scan from the root of the active scene tree.
	_recursive_scan(get_tree().root, map)
	
	# Signal that the scan is complete, passing the resulting map.
	emit_signal("tree_ready", map)

# ----------------------------------------------------------------------------
# UTILITY FUNCTION: Recursively traverses the node tree.
# ----------------------------------------------------------------------------
func _recursive_scan(node: Node, map: Dictionary):
	# Add the current node's name (lowercase for easy lookup) and its full path to the map.
	map[node.name.to_lower()] = node.get_path()
	
	# Recurse through all children.
	for child in node.get_children():
		# Ensure the child is a valid Node before passing it to the function.
		if child is Node:
			_recursive_scan(child, map)