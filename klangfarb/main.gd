tool
extends AudioStreamPlayer
var playback: AudioStreamPlayback

var Buff = preload("res://Main.gdns")
var buff_node = Buff.new()

func _create_generator() -> void:
	stream = AudioStreamGenerator.new()
	stream.mix_rate = 8000.0 # Setting mix rate is only possible before play().
	playback = get_stream_playback()

func _fill_buffer() -> void:
	var to_fill = playback.get_frames_available()
	while to_fill > 0:
		playback.push_frame(Vector2.ONE * buff_node.frames) # Audio frames are stereo.
#		_phase = fmod(_phase + frequency() / GDawConfig.sample_rate, 1.0)
#		_update_state()
		to_fill -= 1


func _ready() -> void:
#	_update_envelope()
	_create_generator()
