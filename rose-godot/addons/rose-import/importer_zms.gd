tool
extends EditorImportPlugin 

const RoseFile = preload("./files/file.gd")
const Utils = preload("utils.gd")
const ZMS = preload("./files/zms.gd")

func get_importer_name():
	return "rose.zms.import"

func get_visible_name():
	return "ROSE Online ZMS"

func get_recognized_extensions():
	return ["zms"]

func get_save_extension():
	return "mesh"

func get_preset_count():
	return 0
	
func get_import_options():
	return null
	
func get_resource_type():
	return "Mesh"
	
func import(src, dst, options, r_platform_variants, r_gen_files):
	var f = RoseFile.new()
	if f.open(src, File.READ) != OK:
		return FAILED
	
	var zms = ZMS.new()
	zms.read(f)
	
	var st = SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	for vi in range(zms.vertices.size()):
		if zms.normals_enabled():
			st.add_normal(zms.vertices[vi].normal)
		if zms.colors_enabled():
			st.add_color(zms.vertices[vi].color)
		if zms.bones_enabled():
			st.add_bones(zms.vertices[vi].bone_indices)
			st.add_weights(zms.vertices[vi].bone_weights)
		if zms.tangents_enabled():
			# TODO: Use `Plane` to correctly load tangent
			st.add_tangent(zms.vertices[vi].tangent)
		if zms.uv1_enabled():
			st.add_uv(zms.vertices[vi].uv1)
		if zms.uv2_enabled():
			st.add_uv2(zms.vertices[vi].uv2)
		
		# Must come last
		if zms.positions_enabled():
			st.add_vertex(Utils.r2g_position(zms.vertices[vi].position))
	
	for i in range(zms.indices.size()):
		st.add_index(zms.indices[i].z)
		st.add_index(zms.indices[i].y)
		st.add_index(zms.indices[i].x)
	
	st.index()
	st.generate_normals()
	st.generate_tangents()
	var mesh = st.commit()
	
	var dds = ""
	var png = ""
	if src.get_extension() == "ZMS":
		dds = src.get_basename() + ".DDS"
		png = src.get_basename() + ".PNG"
	else:
		dds = src.get_basename() + ".dds"
		png = src.get_basename() + ".png"

	var tex = null
	var dir = Directory.new()
	
	# Prefer loading PNG if it exists, otherwise use DDS
	if dir.file_exists(png):
		tex = load(png)
	elif dir.file_exists(dds):
		tex = load(dds)
		
	if tex and mesh.get_surface_count() == 1:
		var mat = SpatialMaterial.new()
		mat.flags_unshaded = true
		mat.albedo_texture = tex

		mesh.surface_set_material(0, mat)

	var file = dst + "." + get_save_extension()
	var err = ResourceSaver.save(file, mesh)
	
	return OK