extends Control

func _gui_input(event):
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT and event.pressed:
			print("I've been clicked D:")
			get_parent().toggle_playback()
		
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.
