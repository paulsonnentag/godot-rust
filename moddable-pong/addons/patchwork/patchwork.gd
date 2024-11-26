@tool
extends EditorPlugin

var file_system_sync: FileSystemSync
var doc: AutomergeDoc

var history_sidebar

func _enter_tree() -> void:
  doc = AutomergeDoc.new()

  # setup file system sync
  file_system_sync = FileSystemSync.new(get_editor_interface().get_resource_filesystem(), doc)

  # add history sidebar
  history_sidebar = preload("res://addons/patchwork/history_sidebar.tscn").instantiate()
  history_sidebar.init(doc)
  add_control_to_dock(DOCK_SLOT_RIGHT_UL, history_sidebar)


func _exit_tree() -> void:
  # Clean up file system sync
  if file_system_sync:
    file_system_sync.destroy()
    file_system_sync = null

  remove_control_from_docks(history_sidebar)