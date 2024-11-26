@tool
extends Control

var doc: AutomergeDoc

@onready var versions_list: ItemList = %VersionsList

func _ready() -> void:
  print("init!!!")
  
func init(doc: AutomergeDoc) -> void:
  self.doc = doc
  doc.changed.connect(_refresh_versions_list)
  
func _refresh_versions_list() -> void:
  if !doc:
    return

  # Clear the existing list
  versions_list.clear()
  
  # Add each version to the list
  for version in doc.history():
    versions_list.add_item(version)
