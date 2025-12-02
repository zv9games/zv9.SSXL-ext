extends Node2D

var init_node: Node = null
var main_node: Node2D = null

var initialized_nodes: Array = []

var is_custom_init_running: bool = false

func _ready():
	call_deferred("_start_custom_initialization")

func _start_custom_initialization():
	if is_custom_init_running:
		return
	is_custom_init_running = true

	init_node = get_node("init")
	main_node = get_node("main")

	if not is_instance_valid(init_node) or not is_instance_valid(main_node):
		return

	init_node.call("initialize")

func report_initialized(nodes: Array) -> void:
	initialized_nodes = nodes

	if is_instance_valid(main_node):
		main_node.call("enter_idle_state") 
	
	is_custom_init_running = false
	if is_instance_valid(init_node):
		pass
