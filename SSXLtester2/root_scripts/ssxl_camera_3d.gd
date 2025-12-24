extends Camera3D

@export var move_speed := 10.0
@export var fast_speed := 40.0
@export var mouse_sensitivity := 0.002
@export var zoom_speed := 5.0

var _yaw := 0.0
var _pitch := 0.0
var _mouse_captured := true


func _ready():
	_capture_mouse(true)


func _capture_mouse(enable: bool):
	_mouse_captured = enable
	if enable:
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	else:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)


func _unhandled_input(event):
	# Toggle mouse capture with SPACE
	if event is InputEventKey and event.pressed and event.keycode == KEY_SPACE:
		_capture_mouse(!_mouse_captured)
		return

	if not _mouse_captured:
		return

	# Mouse look
	if event is InputEventMouseMotion:
		_yaw -= event.relative.x * mouse_sensitivity
		_pitch -= event.relative.y * mouse_sensitivity
		_pitch = clamp(_pitch, -1.5, 1.5)
		rotation = Vector3(_pitch, _yaw, 0)

	# Scroll wheel zoom (local forward/back)
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP and event.pressed:
			translate(Vector3(0, 0, -zoom_speed))
		if event.button_index == MOUSE_BUTTON_WHEEL_DOWN and event.pressed:
			translate(Vector3(0, 0, zoom_speed))


func _process(delta):
	if not _mouse_captured:
		return

	var speed := move_speed
	if Input.is_action_pressed("ui_shift"):
		speed = fast_speed

	var local_dir := Vector3.ZERO

	# Forward/back in LOCAL space
	if Input.is_action_pressed("ui_up"):
		local_dir.z -= 1.0
	if Input.is_action_pressed("ui_down"):
		local_dir.z += 1.0

	# Left/right in LOCAL space
	if Input.is_action_pressed("ui_left"):
		local_dir.x -= 1.0
	if Input.is_action_pressed("ui_right"):
		local_dir.x += 1.0

	# Vertical movement (PageUp/PageDown)
	if Input.is_action_pressed("ui_page_up"):
		local_dir.y += 1.0
	if Input.is_action_pressed("ui_page_down"):
		local_dir.y -= 1.0

	if local_dir != Vector3.ZERO:
		translate(local_dir.normalized() * speed * delta)
