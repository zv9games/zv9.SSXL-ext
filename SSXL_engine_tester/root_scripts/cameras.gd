extends Node

# ----------------------------------------------------------------------
## DEPENDENCIES (using @onready for robust access)
# ----------------------------------------------------------------------

@onready var camera1: Camera2D = $camera1
@onready var camera2: Camera2D = $camera2

# ----------------------------------------------------------------------
## CONSTANTS & STATE
# ----------------------------------------------------------------------

# Zoom constants
const ZOOM_SPEED: float = 0.1 
const MIN_ZOOM: float = 0.05 
const MAX_ZOOM: float = 10.0 

# Panning State
var is_panning: bool = false
var last_mouse_position: Vector2 = Vector2.ZERO

# ----------------------------------------------------------------------
## LIFECYCLE
# ----------------------------------------------------------------------

func _ready() -> void:
	# Ensure camera nodes exist before attempting to use them
	if camera1 == null or camera2 == null:
		push_error("❌ Critical: Camera nodes (camera1 or camera2) not found as children.")
		return
		
	# Start by making camera1 current
	camera1.make_current()

# ----------------------------------------------------------------------
## INPUT HANDLING (Toggle, Zoom, & Pan)
# ----------------------------------------------------------------------

func _unhandled_input(event: InputEvent) -> void:
	# --- 1. Camera Toggle Logic (Space/Tab) ---
	if event is InputEventKey and event.is_pressed() and not event.is_echo():
		if event.is_action("ui_accept") or event.is_action("ui_focus_next"):
			get_viewport().set_input_as_handled()
			_toggle_camera()
			return

	# Only proceed with Mouse/Map controls if camera2 is the current active camera
	if camera2 and camera2.is_current():
		
		# --- 2. Combined Mouse Input Handling (Zoom & Pan State) ---
		if event is InputEventMouseButton:
			
			var current_zoom: Vector2 = camera2.zoom
			var new_zoom: Vector2 = current_zoom
			
			# --- A. ZOOM LOGIC (Scroll Wheel) ---
			# We only check for the press event to prevent double-processing.
			if event.is_pressed():
				if event.button_index == MOUSE_BUTTON_WHEEL_UP:
					# Zoom IN
					new_zoom -= Vector2(ZOOM_SPEED, ZOOM_SPEED)
				elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
					# Zoom OUT
					new_zoom += Vector2(ZOOM_SPEED, ZOOM_SPEED)
			
			if new_zoom != current_zoom:
				# Apply and clamp zoom
				new_zoom = new_zoom.clamp(Vector2(MIN_ZOOM, MIN_ZOOM), Vector2(MAX_ZOOM, MAX_ZOOM))
				camera2.zoom = new_zoom
				get_viewport().set_input_as_handled()
				return 
			
			# --- B. PANNING STATE LOGIC (Middle Click) ---
			if event.button_index == MOUSE_BUTTON_MIDDLE:
				is_panning = event.is_pressed()
				
				if is_panning:
					last_mouse_position = event.position
				
				get_viewport().set_input_as_handled()
				return 

		# --- 3. Camera Pan Logic (Mouse Motion) ---
		elif event is InputEventMouseMotion and is_panning:
			
			var delta: Vector2 = event.position - last_mouse_position
			
			# Correct Pan: Moves the camera by the screen delta adjusted for current zoom level.
			camera2.position -= delta / camera2.zoom
			
			last_mouse_position = event.position
			get_viewport().set_input_as_handled()
			return

# ----------------------------------------------------------------------
## PUBLIC API (No functional change, uses @onready variables)
# ----------------------------------------------------------------------

func switch_to_camera(index: int) -> void:
	if camera1 == null or camera2 == null:
		print("⚠️ Camera nodes not found.")
		return

	if index == 1:
		camera1.make_current()
		print("➡️ Switched to Camera 1 (Control Panel View).")
	elif index == 2:
		camera2.make_current()
		print("➡️ Switched to Camera 2 (Map View).")
	else:
		print("⚠️ Invalid camera index passed.")

func _toggle_camera() -> void:
	if camera1 == null or camera2 == null:
		return

	# Determine which camera is currently active and switch
	if camera1.is_current():
		switch_to_camera(2) # Use public API for consistency
	elif camera2.is_current():
		switch_to_camera(1) # Use public API for consistency
	else:
		# Fallback: if neither is current, default to camera1
		switch_to_camera(1)
