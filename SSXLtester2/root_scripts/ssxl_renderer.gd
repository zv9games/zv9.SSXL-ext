extends SSXLRenderer
class_name SSXLRendererGD

signal chunk_applied(cx, cy)
signal renderer_reset
signal renderer_ready

var _material_applied := false
var _surface_ready_frames := 0
var _atlas_material: StandardMaterial3D = null

func _ready():
	print("SSXLRenderer.gd: ready (Rust backend initialized).")
	print("Renderer World3D:", get_world_3d())

	# Build and cache the material once, keep a strong reference
	var atlas_tex: Texture2D = preload("res://Tileset_atlas_final.png")
	_atlas_material = StandardMaterial3D.new()
	_atlas_material.albedo_texture = atlas_tex
	_atlas_material.texture_filter = BaseMaterial3D.TEXTURE_FILTER_NEAREST
	_atlas_material.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED

	print("DEBUG GD: cached atlas material, tex_rid valid?:",
		atlas_tex.get_rid().is_valid(),
		" mat_rid:", _atlas_material.get_rid()
	)

	if is_inside_tree() and get_world_3d() != null:
		emit_signal("renderer_ready")
	else:
		print("Renderer not fully initialized — skipping ready signal.")


func _process(delta):
	if _material_applied:
		return

	var mesh_rid: RID = get_mesh_rid()
	if not mesh_rid.is_valid():
		return

	var surf_count := RenderingServer.mesh_get_surface_count(mesh_rid)
	if surf_count == 0:
		_surface_ready_frames = 0
		return

	# Surface exists, but may not be fully initialized yet
	if _surface_ready_frames < 2:
		_surface_ready_frames += 1
		return

	var instance_rid: RID = get_instance_rid()
	if not instance_rid.is_valid():
		return

	if _atlas_material == null:
		print("ERROR GD: _atlas_material is null, cannot apply material.")
		return

	var mat_rid: RID = _atlas_material.get_rid()
	print("DEBUG GD: applying cached mat_rid valid?:", mat_rid.is_valid(), " rid=", mat_rid)

	# Attach material at the mesh/surface level
	RenderingServer.mesh_surface_set_material(mesh_rid, 0, mat_rid)

	_material_applied = true
	print("SSXLRendererGD: atlas material applied to mesh RID:", mesh_rid, " instance RID:", instance_rid)


func _apply_atlas_material(instance_rid: RID) -> void:
	var mesh_rid: RID = get_mesh_rid()
	if not mesh_rid.is_valid():
		return

	if _atlas_material == null:
		print("ERROR GD(_apply): _atlas_material is null, cannot apply material.")
		return

	var mat_rid: RID = _atlas_material.get_rid()
	print("DEBUG GD(_apply): reusing cached mat_rid valid?:", mat_rid.is_valid(), " rid=", mat_rid)

	RenderingServer.mesh_surface_set_material(mesh_rid, 0, mat_rid)
