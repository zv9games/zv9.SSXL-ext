extends Node
class_name SSXLRendererView

@export var renderer: SSXLRenderer

func _process(delta: float) -> void:
	if renderer == null:
		return

	var mesh_rid: RID = renderer.get_mesh_rid()
	var instance_rid: RID = renderer.get_instance_rid()

	if mesh_rid.is_valid() and instance_rid.is_valid():
		# Optional: print once for debugging
		#print("RendererView: mesh and instance are valid.")
		set_process(false)
