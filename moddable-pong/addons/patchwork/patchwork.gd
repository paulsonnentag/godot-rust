@tool
extends EditorPlugin

var file_system_sync: FileSystemSync
var doc: AutomergeDoc
var current_file: String

var history_sidebar
func _enter_tree() -> void:


  AutomergeRepo.run()

  # var repo = AutomergeRepo.new()


  # setup file system sync
  # file_system_sync = FileSystemSync.new(get_editor_interface(), doc)

  # # add history sidebar
  # history_sidebar = preload("res://addons/patchwork/history_sidebar.tscn").instantiate()

  # history_sidebar.init(doc)

  # history_sidebar.selected_version.connect(_on_select_version)

  # add_control_to_dock(DOCK_SLOT_RIGHT_UL, history_sidebar)


func _on_select_version(version: String) -> void:
  if !current_file:
    return
    
  # file_system_sync.checkout(current_file, version)


func _process(delta: float) -> void:
  # todo: handle other tabs like scr
  current_file = get_editor_interface().get_edited_scene_root().scene_file_path

func _exit_tree() -> void:
  if file_system_sync:
    file_system_sync.destroy()
    file_system_sync = null

  if history_sidebar:
    remove_control_from_docks(history_sidebar)