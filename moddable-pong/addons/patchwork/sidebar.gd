@tool
extends MarginContainer

var editor_interface: EditorInterface
var undo_redo_manager: EditorUndoRedoManager

@onready var simulated_edits_checkbox: CheckBox = %SimulatedEditsCheckbox

func init(editor_plugin: EditorPlugin) -> void:
  self.editor_interface = editor_plugin.get_editor_interface()
  self.undo_redo_manager = editor_plugin.get_undo_redo()

var last_update_time: int = 0

func _process(_delta: float) -> void:
  if !editor_interface:
    return

  var do_simulated_edits = simulated_edits_checkbox.is_pressed()

  var current_time = Time.get_ticks_msec()
  if do_simulated_edits:
    var paddle = editor_interface.get_edited_scene_root().find_child("BasicPaddleLeft", true, false)

    if paddle:
    
      if (current_time - last_update_time) >= 1000:
        undo_redo_manager.create_action("Rotate paddle randomly")
        undo_redo_manager.add_do_property(paddle, "rotation_degrees", randf_range(-180, 180))
        undo_redo_manager.add_undo_property(paddle, "rotation_degrees", paddle.rotation_degrees)
        undo_redo_manager.commit_action()

        last_update_time = current_time
