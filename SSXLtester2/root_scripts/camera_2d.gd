extends Camera2D

# ============================================================================
# 1. NODE REFERENCES & CONSTANTS
# ============================================================================

# Assuming SSXLSignals is a sibling of the Camera2D node in the main scene.
@onready var ssxl_signals = get_parent().get_node("SSXLSignals") 

# Define the size of one chunk in world units (e.g., 32x32 tiles * 16px/tile = 512)
# Since TileMap coordinates are usually in tiles, we'll use a simpler factor based on
# how chunk coordinates map to world space. Assuming a tile size of 16x16 and 
# a chunk size of 32x32 tiles, the world chunk size is 512.
const WORLD_CHUNK_SIZE: int = 512 

# How many chunks to request in each direction (e.g., RADIUS 2 means 5x5 chunks total).
const STREAM_RADIUS: int = 2 

# Tracks the last chunk coordinate the camera was centered over.
var last_center_chunk: Vector2i = Vector2i(-9999, -9999)


# ============================================================================
# 2. STREAMING LOGIC
# ============================================================================

func _physics_process(delta):
	# Calculate the current chunk key (x, y) based on the camera's position.
	# The integer division by WORLD_CHUNK_SIZE gives us the chunk coordinate.
	var current_center_chunk: Vector2i = (global_position / WORLD_CHUNK_SIZE).floor() as Vector2i
	
	# Only request chunks if the camera has moved into a new central chunk.
	if current_center_chunk != last_center_chunk:
		last_center_chunk = current_center_chunk
		
		if ssxl_signals:
			request_surrounding_chunks(current_center_chunk)
		else:
			push_error("SSXLSignals node is missing or not configured.")

func request_surrounding_chunks(center_key: Vector2i):
	print("--- 📡 Requesting chunks centered at %s ---" % center_key)
	
	# Loop through the streaming radius around the center chunk.
	for x in range(-STREAM_RADIUS, STREAM_RADIUS + 1):
		for y in range(-STREAM_RADIUS, STREAM_RADIUS + 1):
			
			# Calculate the absolute chunk key for the request.
			var request_key = center_key + Vector2i(x, y)
			
			# Emit the signal defined in SSXLSignals.gd.
			# SSXLSignals.gd is set up to listen for this signal and call 
			# SSXLEngine.build_map_by_size(x, y, generator_id).
			ssxl_signals.emit_signal("chunk_request", request_key)
