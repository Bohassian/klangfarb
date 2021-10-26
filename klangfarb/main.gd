extends Node

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
onready var data = preload("res://klangfarbrs.gdnlib")
# var buffer = MonoBuffer.new()

# Called when the node enters the scene tree for the first time.
func _ready():
	print("Poop", data.frames());



# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
