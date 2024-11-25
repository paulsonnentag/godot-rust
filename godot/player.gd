extends Player


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	
	var doc = AutomergeDoc.new();
	

	doc.set("name", "bob");
	
	
	print("name", doc.get("name"));
	
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
