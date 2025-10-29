extends Node2D

var init_node: Node
var main_node: Node2D
var initialized_nodes: Array = []

func _ready():
	get_tree().get_root().print_tree()

	print("🚀 AetherionTester: Summoning subsystems...")

	init_node = get_node("init")
	main_node = get_node("main")

	if init_node == null:
		push_error("❌ AetherionTester: Init node not found.")
		return

	if main_node == null:
		push_error("❌ AetherionTester: Main node not found.")
		return

	print("🧭 AetherionTester: Launching initialization sequence...")
	init_node.call("initialize")

func report_initialized(nodes: Array) -> void:
	initialized_nodes = nodes
	print("✅ AetherionTester: Initialization complete. All nodes accounted for.")
	print("🎙️ AetherionTester: Delegating control to Main...")
	main_node.call("enter_idle_state")
