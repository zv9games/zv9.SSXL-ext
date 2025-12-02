# cameras.gd
# Manages camera switching, map panning, and zooming for the SSXL Tester.
extends Node

# ----------------------------------------------------------------------
## 🧭 NODE REFERENCES (Typed Dependencies)
# ----------------------------------------------------------------------
# We rely on Godot's @onready for robust, typed access to child nodes.
@onready var camera1: Camera2D = $camera1 as Camera2D # Control Panel View
@onready var camera2: Camera2D = $camera2 as Camera2D # Map View (Zoomable, Pannable)

# ----------------------------------------------------------------------
## ⚙️ CONSTANTS & STATE (The Crypto-Coded Memory)
# ----------------------------------------------------------------------

# Zoom constants
const ZOOM_SPEED: float = 0.1
const MIN_ZOOM: float = 0.05
const MAX_ZOOM: float = 10.0

# Panning State
var is_panning: bool = false
var last_mouse_position: Vector2 = Vector2.ZERO


# ----------------------------------------------------------------------
## 🔗 LIFECYCLE (Initialization)
# ----------------------------------------------------------------------

func _ready() -> void:
	# Critical Node Validation
	if not is_instance_valid(camera1) or not is_instance_valid(camera2):
		push_error("❌ CRITICAL: Camera nodes (camera1 or camera2) not found. Check scene tree.")
		return
		
	# Start by ensuring camera1 (the UI view) is active
	camera1.make_current()
	print("✅ Cameras: Initialized. Camera 1 (UI View) is current.")


# ----------------------------------------------------------------------
## ⌨️ INPUT HANDLING (Decoupled Tempo Regulation)
# ----------------------------------------------------------------------

func _unhandled_input(event: InputEvent) -> void:
	
	# Only handle input if the Map Camera (camera2) is currently active
	if not is_instance_valid(camera2) or not camera2.is_current():
		return
		
	# --- 1. Zoom Logic (Mouse Wheel) ---
	if event is InputEventMouseButton:
		var mouse_event: InputEventMouseButton = event
		
		# Zoom In
		if mouse_event.button_index == MOUSE_BUTTON_WHEEL_UP:
			var new_zoom: float = camera2.zoom.x + ZOOM_SPEED
			set_map_zoom(new_zoom)
			get_viewport().set_input_as_handled()
		
		# Zoom Out
		elif mouse_event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			var new_zoom: float = camera2.zoom.x - ZOOM_SPEED
			set_map_zoom(new_zoom)
			get_viewport().set_input_as_handled()
		
		# Panning Start/Stop (Middle Mouse Button)
		elif mouse_event.button_index == MOUSE_BUTTON_MIDDLE:
			is_panning = mouse_event.is_pressed()
			
			if is_panning:
				last_mouse_position = mouse_event.position
			
			get_viewport().set_input_as_handled()
			return
	
	# --- 2. Camera Pan Logic (Mouse Motion) ---
	elif event is InputEventMouseMotion and is_panning:
		var delta: Vector2 = event.position - last_mouse_position
		
		# Correct Pan: Moves the camera by the screen delta adjusted for current zoom level.
		# This must be inverted to make drag feel natural.
		camera2.position -= delta / camera2.zoom
		
		last_mouse_position = event.position
		get_viewport().set_input_as_handled()
		return
		
	# --- 3. Camera Toggle Logic (Keyboard Shortcut - Delegated) ---
	# NOTE: The ControlPanelUtility script is the primary handler for F6/Tab,
	# calling the _toggle_camera() method below. This keeps input centralized.


# ----------------------------------------------------------------------
## 🖼️ CAMERA UTILITIES (Map Control)
# ----------------------------------------------------------------------

func set_map_zoom(target_zoom: float) -> void:
	"""Clamps the zoom level and applies it to the map camera (camera2)."""
	
	if not is_instance_valid(camera2):
		return
		
	# Clamp the zoom level to prevent extreme values (systemic entropy)
	var clamped_zoom: float = clampf(target_zoom, MIN_ZOOM, MAX_ZOOM)
	
	# Apply zoom uniformly across X and Y axes
	camera2.zoom = Vector2(clamped_zoom, clamped_zoom)


# ----------------------------------------------------------------------
## 📡 PUBLIC API (The FFI Link for the Control Panel)
# ----------------------------------------------------------------------

func switch_to_camera(index: int) -> void:
	"""Makes the specified camera current (1: UI, 2: Map)."""
	
	if not is_instance_valid(camera1) or not is_instance_valid(camera2):
		push_error("⚠️ Camera nodes not found.")
		return

	if index == 1:
		camera1.make_current()
		print("➡️ Switched to Camera 1 (Control Panel View).")
	elif index == 2:
		camera2.make_current()
		print("➡️ Switched to Camera 2 (Map View).")
	else:
		push_warning("⚠️ Invalid camera index passed.")


func _toggle_camera() -> void:
	"""Switches between the two available cameras."""
	if not is_instance_valid(camera1) or not is_instance_valid(camera2):
		return

	# Determine which camera is currently active and switch
	if camera1.is_current():
		switch_to_camera(2) # Switch to Map View
	elif camera2.is_current():
		switch_to_camera(1) # Switch to Control Panel View
