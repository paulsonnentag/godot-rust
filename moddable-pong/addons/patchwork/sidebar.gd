@tool
extends MarginContainer

var editor_interface: EditorInterface

@onready var simulated_edits_checkbox: CheckBox = %SimulatedEditsCheckbox

func init(editor_interface: EditorInterface) -> void:
  self.editor_interface = editor_interface


var last_save_time: int = 0

func _process(_delta: float) -> void:
  if !editor_interface:
    return

  var do_simulated_edits = simulated_edits_checkbox.is_pressed()

  var current_time = Time.get_ticks_msec()
  if do_simulated_edits:
    var paddle = editor_interface.get_edited_scene_root().find_child("BasicPaddleLeft", true, false)

    if paddle:
    
      # Save every 2000ms (2 seconds)
      if (current_time - last_save_time) >= 1000:
        # Generate random rotation between -180 and 180 degrees
        paddle.rotation_degrees = randf_range(-180, 180)

        last_save_time = current_time
        editor_interface.save_scene()
