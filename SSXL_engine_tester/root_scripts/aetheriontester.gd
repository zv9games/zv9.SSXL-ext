extends Node

@onready var scanner = $/root/aetheriontester/scenescanner
@onready var main = $/root/aetheriontester/main
@onready var init = $/root/aetheriontester/init

var scenemap = {}

func _ready():
	print("🌌 aetheriontester initializing...")
#	scanner.connect("tree_ready", self._on_tree_ready)
#	scanner.scan_tree()

func _on_tree_ready(map):
	scenemap = map
	#print("✅ scene tree scan complete. nodes found:")
	for name in scenemap.keys():
		#print(" - %s: %s" % [name, scenemap[name]])
		pass
	
	init.initialize(scenemap, main)
