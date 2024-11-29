@tool
class_name FileSystemSync

var editor_interface: EditorInterface
var file_system: EditorFileSystem
#var doc: AutomergeDoc

var listener_disabled = false

func _init(editor_interface: EditorInterface):
  self.editor_interface = editor_interface
  file_system = editor_interface.get_resource_filesystem()
  #self.doc = doc

  print("fs", file_system)

  file_system.connect("filesystem_changed", _on_filesystem_changed)
  file_system.connect("resources_reload", _on_resources_reloaded)

func destroy():
  # Cleanup connections when plugin is disabled
  if file_system:
      file_system.disconnect("filesystem_changed", _on_filesystem_changed)
      file_system.disconnect("resources_reload", _on_resources_reloaded)

# todo: change the other files as well
func checkout(path: String, version: String):
  pass
  # var file = FileAccess.open(path, FileAccess.WRITE)
  # if not file:
  #   return
    
  # # Get the content at the specified version
  # var content = doc.get_at(path, version)
  # if not content:
  #   return
    
  # # Write the content to the file
  # file.store_string(content)
  # file.close()
  
  # # Trigger filesystem scan to detect changes

  
  # listener_disabled = true


  # print("reload", path)
  # editor_interface.reload_scene_from_path(path)

  # listener_disabled = false

  
func _on_filesystem_changed():
  if listener_disabled:
    return

  # This is called when any file system changes are detected
  scan_for_changes()

func _on_resources_reloaded(resources: Array):
  # This is called when specific resources are reloaded
  for path in resources:
      _check_file_changes(path)

func scan_for_changes():
  # Scan the entire project directory
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
  pass
  # # hack: ignore all files that are not main.tscn
  # if not file_path.ends_with("main.tscn"):
  #   return

  # # Skip files that aren't GDScript or scene file
  # if not file_path.ends_with(".gd") and not file_path.ends_with(".tscn"):
  #   return

  # var file = FileAccess.open(file_path, FileAccess.READ)
  # if not file:
  #     return
  
  # var content = file.get_as_text(true)
  # var stored_content = doc.get(file_path)

  # if content != stored_content:
  #   _handle_file_changed(file_path, content)
    
func _handle_file_changed(file_path: String, content: String):
  pass
  # print("File changed: ", file_path)
  # doc.set(file_path, content)
