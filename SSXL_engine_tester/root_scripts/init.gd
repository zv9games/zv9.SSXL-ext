extends Node

# Stores a reference to every Godot Node found during the recursive scan.
var initialized_nodes: Array[Node] = []

# ----------------------------------------------------------------------------
# CORE FUNCTION: Starts the recursive node tree scan.
# This is called explicitly by the root orchestrator (ssxltester).
# ----------------------------------------------------------------------------
func initialize():
	print("\n🎶 Init: Commencing recursive scene scan...")
	initialized_nodes.clear()

	# Get the root viewport node to start the traversal.
	var root_node := get_tree().get_root()
	if root_node == null:
		push_error("❌ Init: Could not access scene root. Ritual aborted.")
		return

	_recursive_initialize(root_node)

	# Locate the root orchestration node to deliver the completion report.
	var tester := root_node.get_node("ssxltester")
	# NOTE: Changed "AetherionTester" to "ssxltester" for project consistency.
	if tester == null:
		push_error("❌ Init: ssxltester node not found. No one to receive the scroll.")
		return

	print("\n📜 Init: Scroll prepared. Delivering to SSXLTester...")
	
	# Call the callback function on the root node to signal completion.
	tester.call("report_initialized", initialized_nodes)

# ----------------------------------------------------------------------------
# UTILITY FUNCTION: Recursively collects all nodes starting from the given node.
# ----------------------------------------------------------------------------
func _recursive_initialize(node: Node) -> void:
	# Add the current node to the collection array.
	initialized_nodes.append(node)

	# Traverse through all child nodes.
	for child in node.get_children():
		# Ensure the child is a valid Node instance before recursing.
		if child is Node:
			_recursive_initialize(child)
