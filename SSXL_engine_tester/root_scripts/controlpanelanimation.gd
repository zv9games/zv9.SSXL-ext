class_name control_panel_animation_logic extends RefCounted

## 🎯 Properties
var controller: Control
# Queue to hold incoming tile updates (flips) from the engine
var tile_flip_queue: Array = []
# Maximum number of tile updates to process per animation tick
const MAX_FLIPS_PER_TICK: int = 1000

## 🛠️ Internal Command Names (Must match Rust FFI API)
const CMD_ANIMATION_ENABLE: String = "ANIMATION_ENABLE"
const CMD_ANIMATION_DISABLE: String = "ANIMATION_DISABLE"


func _init(p_controller: Control) -> void:
	# Store a reference to the main controller node
	controller = p_controller



##  ANIMATION STATE CONTROL

func on_animate_checkbox_toggled(button_pressed: bool) -> void:
	"""
	Updates the controller's local animation state and sends the new mode 
	to the Rust engine via FFI to update its status.
	"""
	controller.is_animated = button_pressed
	print("⚙️ Animation Toggled: %s" % controller.is_animated)
	
	# FFI: Send the correct command string to the engine.
	if is_instance_valid(controller.ssxl_engine):
		var command_to_send: String = CMD_ANIMATION_ENABLE if button_pressed else CMD_ANIMATION_DISABLE
		controller.ssxl_engine.send_animation_command(command_to_send)
	
	# Automatically switch the polling mechanism based on the animation mode
	setup_animation_worker(button_pressed)


func setup_animation_worker(should_animate: bool) -> void:
	"""
	Starts the correct timer (throttled animation or fast engine poll)
	based on the user's animation choice.
	"""
	if should_animate:
		if is_instance_valid(controller.animation_timer):
			# Slower tick for visual updates
			controller.animation_timer.start()
		if is_instance_valid(controller.engine_timer):
			# Stop the fast poll
			controller.engine_timer.stop()
	else:
		if is_instance_valid(controller.engine_timer):
			# Fast tick for engine polling
			controller.engine_timer.start()
		if is_instance_valid(controller.animation_timer):
			# Stop the animation loop
			controller.animation_timer.stop()



## ⏱️ TIMER HANDLERS (Polling & Animation Loop)

func _on_engine_timer_timeout() -> void:
	"""
	Called by the fast (e.g., 0.01s) engine_timer when NOT animating.
	Its primary job is to poll the Rust engine for updates.
	"""
	if not controller.is_generating or not is_instance_valid(controller.ssxl_engine):
		return
	
	controller.engine_tick_count += 1
	# Manually poll the engine status
	controller.ssxl_engine.get_status()


func _on_animation_timer_timeout() -> void:
	"""
	Called by the throttled animation_timer when ANIMATING.
	Processes a batch of tile flips and updates the screen once.
	"""
	if not controller.is_generating:
		return
	
	# 1. Determine how many flips to process
	var flips_to_process: int = min(tile_flip_queue.size(), MAX_FLIPS_PER_TICK)
	
	if flips_to_process > 0:
		# Process the first batch of flips directly from the queue
		for i in range(flips_to_process):
			var flip_data: Array = tile_flip_queue[i]
			var tile_id: int = flip_data[0]
			var flip_frame: int = flip_data[1]
			
			process_tile_flip(tile_id, flip_frame)
			
		# FIX: Remove the processed elements from the queue using pop_front()
		# This is the correct way to handle FIFO batch removal in Godot 4.x.
		for i in range(flips_to_process):
			tile_flip_queue.pop_front()


	# 2. Update Map and Poll Engine Status
	if is_instance_valid(controller.expansive_tilemap):
		# Force the TileMap to refresh with the batch of new tiles
		controller.expansive_tilemap.force_update()
	
	# Poll Engine Status (Less frequently in animation mode)
	if is_instance_valid(controller.ssxl_engine):
		controller.ssxl_engine.get_status()



## 💡 TILE FLIP PROCESSING

func process_tile_flip(tile_id: int, flip_frame: int) -> void:
	"""Converts linear tile ID to grid coords and applies the visual flip/frame."""
	if not is_instance_valid(controller.expansive_tilemap) or not is_instance_valid(controller.grid_width):
		return

	# Use the current grid width value (which is likely a SpinBox or similar)
	var map_width: int = int(controller.grid_width.value)
	if map_width <= 0:
		return

	# Convert linear tile_id back to grid coordinates (x, y)
	var x: int = tile_id % map_width
	var y: int = tile_id / map_width
	var coords: Vector2i = Vector2i(x, y)
	
	# Apply the tile flip/alternative tile index
	controller.expansive_tilemap.set_cell_alt(0, coords, flip_frame)


func _on_tile_flip_updated(tile_id: int, flip_frame: int) -> void:
	"""
	Signal handler connected to SSXLSignals.
	Queues the incoming tile flip data for processing on the next animation tick.
	"""
	# Only queue updates if the system is running and animation is enabled
	if not controller.is_animated or not controller.is_generating:
		return

	# Store the data in the queue immediately
	tile_flip_queue.append([tile_id, flip_frame])
