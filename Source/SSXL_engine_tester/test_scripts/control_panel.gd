extends Node

func _ready():
	# Wire up dependencies
	$AetherionEngine.signals_node = $AetherionSignals

	$AetherionEngine.set_tilemap($TileMap)

	# Connect signals
	$AetherionSignals.connect("generation_progress", Callable(self, "_on_progress"))
	$AetherionSignals.connect("generation_complete", Callable(self, "_on_complete"))
	$AetherionSignals.connect("map_building_status", Callable(self, "_on_status"))


	# Trigger map generation
	var seed = randi()
	$AetherionEngine.build_map(
		32, 32, seed, "basic", true,
		Vector2i(0, 0), Vector2i(1, 0)
	)

func _on_progress(percent):
	print("Progress:", percent)

func _on_complete(results):
	print("Map generation complete:")
	for key in results.keys():
		print("  %s: %s" % [key, results[key]])

func _on_status(msg):
	print("Status:", msg)
