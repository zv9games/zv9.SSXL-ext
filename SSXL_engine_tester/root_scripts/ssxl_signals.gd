
extends Node

# ==============================================================================
# FFI Data Plane Signals (Emitted by the Rust GDExtension)
# These signals MUST be declared here to be visible to GDScript.
# ==============================================================================

# 1. PRIMARY DATA CHANNEL (CRITICAL FIX)
signal chunk_ready(data: Dictionary) 

# 2. GENERATION COMPLETION SIGNAL
signal build_map_complete()
# NOTE: The FFI manifest calls this 'generation_complete', but your GDScript checks for 'build_map_complete'.

# 3. STATUS / LOGGING SIGNAL
signal engine_status_updated(status: String)
# NOTE: The FFI manifest calls this 'status_update', but your GDScript checks for 'engine_status_updated'.

# 4. ANIMATION/TICKER SIGNAL
signal tile_flip_updated(tile_id: int, flip_frame: int)

# The Rust GDExtension is responsible for emitting these signals; 
# this script just makes them visible to the Godot environment.
