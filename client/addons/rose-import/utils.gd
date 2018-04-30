tool

static func r2g_position(pos):
	# Converts ROSE (Z-UP) position to Godot (Y-UP) position
	return Vector3(pos.x, pos.z, pos.y)

static func r2g_rotation(rot):
	# Converts ROSE (Z-UP) quaternion to Godot (Y-UP) quaternion
	return Quat(rot.x, rot.z, rot.y, -rot.w)