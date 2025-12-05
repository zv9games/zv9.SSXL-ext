# main.gd
extends Node
class_name SSXLRoot

@onready var engine: SSXLEngine = $SSXLENGINE
@onready var signals: SSXLSignals = $SSXLSignals
@onready var tilemap: SSXLTilemap = $SSXLTilemap
@onready var camera: SSXLCameraDriver = $Camera2D

func _ready():
	# Connect Camera driver to Engine request
	signals.chunk_request.connect(engine.request_chunk)
	
	# Trigger initial load via Camera's first _physics_process
	print("SSXL System online. FFI -> Signals -> TileMap data flow established.")
