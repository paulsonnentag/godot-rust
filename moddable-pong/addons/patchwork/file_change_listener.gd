@tool
class_name FileChangeListener

var file_system: EditorFileSystem
var file_contents: Dictionary = {}

signal file_changed(path: String, file_name: String)

func _init(file_system: EditorFileSystem):
  self.file_system = file_system

  print("start fs listener")

  file_system.connect("filesystem_changed", _on_filesystem_changed)
  file_system.connect("resources_reload", _on_resources_reloaded)

func stop():
  # Cleanup connections when plugin is disabled
  if file_system:
      file_system.disconnect("filesystem_changed", _on_filesystem_changed)
      file_system.disconnect("resources_reload", _on_resources_reloaded)
  
func _on_filesystem_changed():
  _scan_for_changes()

func _on_resources_reloaded(resources: Array):
  for path in resources:
      _check_file_changes(path)

func _scan_for_changes():
  var dir = DirAccess.open("res://")
  if dir:
      _scan_directory(dir, "res://")

func _scan_directory(dir: DirAccess, current_path: String):
  # Recursively scan directories for files
  dir.list_dir_begin()
  var file_name = dir.get_next()
  
  while file_name != "":
      if file_name == "." or file_name == "..":
          file_name = dir.get_next()
          continue
          
      var full_path = current_path.path_join(file_name)
      
      if dir.current_is_dir():
          var sub_dir = DirAccess.open(full_path)
          if sub_dir:
              _scan_directory(sub_dir, full_path)
      else:
          _check_file_changes(full_path)
          
      file_name = dir.get_next()

func _check_file_changes(file_path: String):
  # Skip files that aren't GDScript or scene file
  if not file_path.ends_with(".gd") and not file_path.ends_with(".tscn"):
    return

  var file = FileAccess.open(file_path, FileAccess.READ)
  if not file:
      return
  
  var content = file.get_as_text(true)
  var stored_content = file_contents.get(file_path, "")

  if content != stored_content:
    file_contents[file_path] = content
    file_changed.emit(file_path, content)
