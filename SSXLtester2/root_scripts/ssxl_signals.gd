# SSXLSignals.gd (Updated to match the API Rule Book)
extends Node

# --- A. GENERATION LIFECYCLE SIGNALS ---
signal build_map_start
signal build_map_complete
signal build_map_stopped
signal generation_error(error_message: String)

# --- B. DATA TRANSFER SIGNALS (The Critical Change) ---
# These are the PULL signals (coordinates only, Godot retrieves the data)
signal chunk_generated(x: int, y: int) 
signal chunk_data_ready(x: int, y: int) 

# --- C. UTILITY SIGNALS ---
signal tick_complete(current_tick: int)
signal tile_flip_updated(tile_id: int, flip_frame: int)
signal animation_update(percent_done: float, new_atlas_coords: Vector2i)
signal animation_state_changed(enabled: bool)
signal engine_status_updated(status_message: String) # Note the name change from 'status_update'

func _ready():
	print("SSXLSignals node is ready and signals are declared.")
