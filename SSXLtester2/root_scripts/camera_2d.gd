extends Camera2D

@export var zoom_factor: float = 1.25  # 25% zoom per wheel notch
@export var min_zoom: float = 0.005    # <<< FIX: Lowered to allow massive zoom-out
@export var max_zoom: float = 64.0

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		var pivot := global_position  # zoom toward mouse cursor
		
		match event.button_index:
			MOUSE_BUTTON_WHEEL_UP:
				zoom *= zoom_factor
			MOUSE_BUTTON_WHEEL_DOWN:
				zoom /= zoom_factor
		
		# Clamping now allows for far greater zoom-out
		zoom = Vector2.ONE * clamp(zoom.x, min_zoom, max_zoom)
		
		# Keep mouse-pointed world position fixed
		global_position = pivot + (global_position - pivot) * (zoom.x / (zoom.x / zoom_factor if event.button_index == MOUSE_BUTTON_WHEEL_UP else zoom.x * zoom_factor))
