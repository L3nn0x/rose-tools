tool
extends EditorPlugin

var zmd_importer = null
var zmo_importer = null
var zms_importer = null

func _enter_tree():
	zmd_importer = preload("importer_zmd.gd").new()
	zmo_importer = preload("importer_zmo.gd").new()
	zms_importer = preload("importer_zms.gd").new()
	add_import_plugin(zmd_importer)
	add_import_plugin(zmo_importer)
	add_import_plugin(zms_importer)

func _exit_tree():
	remove_import_plugin(zmd_importer)
	remove_import_plugin(zmo_importer)
	remove_import_plugin(zms_importer)
	zmd_importer = null
	zmo_importer = null
	zms_importer = null