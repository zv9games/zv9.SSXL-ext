extends Node2D

var init_node: Node
var main_node: Node2D
var initialized_nodes: Array = []

func _ready():
	#get_tree().get_root().print_tree()

	print("🚀 SSXLTester: Summoning subsystems...")

	init_node = get_node("init")
	main_node = get_node("main")

	if init_node == null:
		push_error("❌ SSXLTester: Init node not found.")
		return

	if main_node == null:
		push_error("❌ SSXLTester: Main node not found.")
		return

	print("🧭 SSXLTester: Launching initialization sequence...")
	init_node.call("initialize")

func report_initialized(nodes: Array) -> void:
	initialized_nodes = nodes
	print("✅ SSXLTester: Initialization complete. All nodes accounted for.")
	print("🎙️ SSXLTester: Delegating control to Main...")
	main_node.call("enter_idle_state")
