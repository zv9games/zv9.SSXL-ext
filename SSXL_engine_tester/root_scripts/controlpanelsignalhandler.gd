class_name control_panel_signal_handler extends RefCounted

var controller: Control

func _init(p_controller: Control) -> void:
	# Store the reference to the main ControlPanel node
	controller = p_controller

## 🔄 ENGINE STATUS/ERROR SIGNALS
# 
func _on_engine_status_updated(status_message: String) -> void:
	"""Updates the status label with a general message from the engine."""
	if is_instance_valid(controller.status_label):
		# Only update the status if it's a critical or final state, 
		# allowing chunk_generated to handle progress status.
		if status_message.begins_with("ERROR:") or status_message.begins_with("IDLE:"):
			controller.status_label.text = status_message

func _on_generation_error(error_message: String) -> void:
	"""Handles a critical error signal from the generation process."""
	print("❌ ERROR: Generation failed: %s" % error_message)
	
	controller.utility.reset_temporary_state()
	
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "❌ ERROR: Generation failed. Check console."
	if is_instance_valid(controller.generate_button):
		controller.generate_button.text = "IGNITION"


## 🏗️ GENERATION PROGRESS SIGNALS
func _on_build_map_start() -> void:
	"""Fired when the FFI build_map call is successfully received by the Rust engine."""
	print("📡 Signal: build_map_start received.")
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "🏗️ Engine task started..."

func _on_chunk_generated(chunk_x: int, chunk_y: int) -> void:
	"""Updates progress bar and tile count when a chunk is processed by the engine."""
	var tiles_in_chunk: int = controller.CHUNK_SIZE_X * controller.CHUNK_SIZE_Y
	
	# Update Progress Bar and Tile Count
	if is_instance_valid(controller.progress_bar):
		controller.progress_bar.value += float(tiles_in_chunk)
	
	controller.total_tiles_placed += tiles_in_chunk
	
	if is_instance_valid(controller.tiles_placed_label):
		controller.tiles_placed_label.text = "Tiles Placed: %d" % controller.total_tiles_placed
	
	# Request the utility module to schedule a visual TileMap redraw (throttled)
	# This is the key to fixing the map refresh rate.
	controller.utility.request_redraw()

	# Update the status label with percentage progress
	var percent: int = 0
	if is_instance_valid(controller.progress_bar) and controller.progress_bar.max_value > 0:
		percent = int(controller.progress_bar.value / controller.progress_bar.max_value * 100.0)

	if is_instance_valid(controller.status_label):
		# Only update status if the percentage value has changed significantly (e.g., 5%)
		if percent != controller.last_percent and (percent % 5 == 0 or percent == 100):
			controller.status_label.text = "🏗️ Chunk (%d, %d) applied... %d%%" % [chunk_x, chunk_y, percent]
		
	controller.last_percent = percent


## 💡 ANIMATION/TILE FLIP SIGNAL
# 
func _on_tile_flip_updated(tile_id: int, flip_frame: int) -> void:
	"""
	Handles an individual tile flip update signal, delegating the data to the
	Animation Logic module for queueing and processing.
	"""
	if not is_instance_valid(controller.animation_logic):
		return
	
	# FIX: Delegate the flip data to the animation logic module for queueing.
	controller.animation_logic._on_tile_flip_updated(tile_id, flip_frame)
