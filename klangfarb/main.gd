tool
extends AudioStreamPlayer
var playback: AudioStreamPlayback = null

var MonoBuffer = preload("res://MonoBuffer.gdns")
var buffer = MonoBuffer.new()
var samples = buffer.frames
var sample_count = samples.size()


func _create_generator() -> void:
	stream = AudioStreamGenerator.new()
	stream.mix_rate = 8000.0 # Setting mix rate is only possible before play().
	playback = get_stream_playback()

func _fill_buffer() -> void:
	var to_fill = playback.get_frames_available()
	while to_fill > 0:
		var sample_index = to_fill % 512
		playback.push_frame(Vector2.ONE * samples[sample_index]) # Audio frames are stereo.
#		_phase = fmod(_phase + frequency() / GDawConfig.sample_rate, 1.0)
#		_update_state()
		to_fill -= 1

func _process(delta):
	_fill_buffer()

func _ready() -> void:
	_create_generator()
	print(sample_count)
	_fill_buffer()
	self.play()
#	_update_envelope()

