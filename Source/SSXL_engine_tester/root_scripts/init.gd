extends Node

var initialized_nodes: Array = []

func initialize():
	print("\n🎶 Init: Commencing recursive scene scan...")
	initialized_nodes.clear()

	var root_node := get_tree().get_root()
	if root_node == null:
		push_error("❌ Init: Could not access scene root. Ritual aborted.")
		return

	_recursive_initialize(root_node)

	var tester := root_node.get_node("aetheriontester")
	if tester == null:
		push_error("❌ Init: AetherionTester node not found. No one to receive the scroll.")
		return

	print("\n📜 Init: Scroll prepared. Delivering to AetherionTester...")
	tester.call("report_initialized", initialized_nodes)

func _recursive_initialize(node: Node) -> void:
	print("✨ Init: Blessing node → %s" % node.name)
	initialized_nodes.append(node)

	for child in node.get_children():
		if child is Node:
			_recursive_initialize(child)
