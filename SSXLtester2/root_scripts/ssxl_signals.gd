# signals.gd
extends Node

# Fired by Camera, caught by Engine (O(1) request)
signal chunk_request(chunk_key: Vector2i) 

# Fired by Engine, caught by TileMap (O(1) dispatch)
signal chunk_loaded(key: Vector2i, data: Dictionary)
