@tool
extends EditorPlugin

var file_system_sync: FileSystemSync

func _enter_tree() -> void:
	# Initialize the file system sync
	file_system_sync = FileSystemSync.new(get_editor_interface().get_resource_filesystem())


func _exit_tree() -> void:
	# Clean up file system sync
	if file_system_sync:
		file_system_sync.destroy()
		file_system_sync = null
