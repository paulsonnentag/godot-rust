@tool
extends EditorPlugin

var file_change_listener: FileChangeListener
var automerge_fs: AutomergeFS
var history_sidebar

func _enter_tree() -> void:

  # /efc9/08d79d8e432046c0b8df0e320d5edf0
  automerge_fs = AutomergeFS.create("08d79d8e432046c0b8df0e320d5edf0b")
  automerge_fs.start();

  # listen to remove changes
  automerge_fs.file_changed.connect(_on_remote_file_changed)

  # setup file system sync
  file_change_listener = FileChangeListener.new(get_editor_interface().get_resource_filesystem())
  file_change_listener.file_changed.connect(_on_local_file_changed)


func _on_local_file_changed(path: String, content: String) -> void:
  # for now ignore all files that are not main.tscn
  if not path.ends_with("main.tscn"):
    return

  print("file changed ", path);
  automerge_fs.save(path, content);

func _on_remote_file_changed(path: String, content: String) -> void:
  # for now ignore all files that are not main.tscn
  if not path.ends_with("main.tscn"):
    return

  print("remote file changed ", path);


func _process(delta: float) -> void:
  if automerge_fs:
    automerge_fs.refresh();

func _exit_tree() -> void:
  if history_sidebar:
    remove_control_from_docks(history_sidebar)

  if automerge_fs:
    automerge_fs.stop();

  if file_change_listener:
    file_change_listener.stop()
