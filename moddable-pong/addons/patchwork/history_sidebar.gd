@tool
extends Control

signal selected_version(version: String)

# var doc: AutomergeDoc

@onready var versions_list: ItemList = %VersionsList

func _ready() -> void:
  versions_list.item_selected.connect(_on_version_selected)
  
func init() -> void:
  pass
  #self.doc = doc
  #doc.changed.connect(_refresh_versions_list)
  
func _refresh_versions_list() -> void:
  pass
  # #if !doc:
  #   return

  # # Clear the existing list
  # versions_list.clear()
  
  # # Add each version to the list
  # for version in doc.history():
  #   versions_list.add_item(version)

func _on_version_selected(index: int) -> void:
  selected_version.emit(versions_list.get_item_text(index))
