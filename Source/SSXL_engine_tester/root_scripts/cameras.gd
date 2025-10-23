extends Node

# ----------------------------------------------------------------------
## CONSTANTS & STATE
# ----------------------------------------------------------------------

# Zoom constants
const ZOOM_SPEED: float = 0.1 # How fast the zoom changes
const MIN_ZOOM: float = 0.05 # Zoomed way in (5% of standard view) <-- INCREASED ZOOM IN
const MAX_ZOOM: float = 10.0 # Zoomed way out (10x standard view) <-- INCREASED ZOOM OUT

# Panning State
var is_panning: bool = false
var last_mouse_position: Vector2 = Vector2.ZERO

# ----------------------------------------------------------------------
## LIFECYCLE
# ----------------------------------------------------------------------

func _ready() -> void:
	# Ensure one camera is current on start. Default to camera1.
	var camera1: Camera2D = $camera1
	var camera2: Camera2D = $camera2
	
	if camera1:
		camera1.make_current()
	elif camera2:
		camera2.make_current()

# ----------------------------------------------------------------------
## INPUT HANDLING (Toggle, Zoom, & Pan)
# ----------------------------------------------------------------------

func _unhandled_input(event: InputEvent) -> void:
	# Reference to camera2 (used in multiple places)
	var camera2: Camera2D = $camera2
	
	# Only proceed if camera2 exists AND is the current active camera
	if camera2 and camera2.is_current():
		
		# --- 1. Combined Mouse Input Handling (Zoom & Pan State) ---
		if event is InputEventMouseButton:
			
			var current_zoom: Vector2 = camera2.zoom
			var new_zoom: Vector2 = current_zoom
			var should_handle_input: bool = false
			
			# --- ZOOM LOGIC (Check Scroll Wheel FIRST) ---
			if event.is_pressed(): # Only process zoom on the press event
				if event.button_index == MOUSE_BUTTON_WHEEL_UP:
					# Zoom IN (towards 0.0)
					new_zoom -= Vector2(ZOOM_SPEED, ZOOM_SPEED) 
					should_handle_input = true
					
				elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
					# Zoom OUT (towards MAX_ZOOM)
					new_zoom += Vector2(ZOOM_SPEED, ZOOM_SPEED) 
					should_handle_input = true

			if should_handle_input:
				# Apply and clamp zoom using the new MIN/MAX limits
				new_zoom = new_zoom.clamp(Vector2(MIN_ZOOM, MIN_ZOOM), Vector2(MAX_ZOOM, MAX_ZOOM))
				camera2.zoom = new_zoom
				get_viewport().set_input_as_handled()
				return # Input handled, exit the function
			
			# --- PANNING STATE LOGIC (Check Middle Click AFTER Zoom) ---
			if event.button_index == MOUSE_BUTTON_MIDDLE: # Scroll Wheel Click
				if event.is_pressed():
					# Start Panning
					is_panning = true
					last_mouse_position = event.position
					get_viewport().set_input_as_handled()
				else:
					# Stop Panning
					is_panning = false
					get_viewport().set_input_as_handled()
				return # Input handled, exit the function

		# --- 2. Camera Pan Logic (Mouse Motion) ---
		elif event is InputEventMouseMotion and is_panning:
			
			var delta: Vector2 = event.position - last_mouse_position
			
			# Pan the camera in the opposite direction
			camera2.position -= delta * camera2.zoom
			
			last_mouse_position = event.position
			get_viewport().set_input_as_handled()
			return # Handled mouse motion, exit early

	# --- 3. Camera Toggle Logic (using Space/Tab) ---
	# This logic must be checked even if camera2 is NOT current.
	if event is InputEventKey and not event.is_echo():
		if event.is_action_pressed("ui_accept") or event.is_action_pressed("ui_focus_next"):
			
			get_viewport().set_input_as_handled()
			_toggle_camera()
			return
			
# ----------------------------------------------------------------------
## PUBLIC API (Remains the same)
# ----------------------------------------------------------------------

func switch_to_camera(index: int) -> void:
	var cam1: Camera2D = $camera1
	var cam2: Camera2D = $camera2

	if cam1 == null or cam2 == null:
		print("⚠️ Camera nodes not found.")
		return

	if index == 1:
		cam1.make_current()
		print("➡️ Switched to Camera 1 (Control Panel View).")
	elif index == 2:
		cam2.make_current()
		print("➡️ Switched to Camera 2 (Map View).")
	else:
		print("⚠️ Invalid camera index passed.")

func _toggle_camera() -> void:
	var camera1: Camera2D = $camera1
	var camera2: Camera2D = $camera2

	if camera1 == null or camera2 == null:
		print("⚠️ Cameras not found.")
		return

	# Determine which camera is currently active and switch
	if camera1.is_current():
		camera2.make_current()
		print("➡️ Switched to Camera 2 (Map View).")
	elif camera2.is_current():
		camera1.make_current()
		print("⬅️ Switched to Camera 1 (Control Panel View).")
	else:
		# Fallback: if neither is current, default to camera1
		camera1.make_current()
		print("🔄 No camera was active, defaulting to Camera 1.")
