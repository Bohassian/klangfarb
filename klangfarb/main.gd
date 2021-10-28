extends AudioStreamPlayer
var playback: AudioStreamPlayback = null

var MonoBuffer = preload("res://MonoBuffer.gdns")
var buffer = MonoBuffer.new()
var samples = buffer.frames
var sample_count = samples.size()
var current_count = 0

#func _create_generator() -> void:
#	stream = AudioStreamGenerator.new()
#	stream.mix_rate = 44100.0 # Setting mix rate is only possible before play().
#	playback = get_stream_playback()

func _fill_buffer() -> void:
	var to_fill = playback.get_frames_available()
	
	while to_fill > 0 && current_count != sample_count:
		var sample_index = current_count
		playback.push_frame(Vector2.ONE * samples[sample_index]) # Audio frames are stereo.
#		_phase = fmod(_phase + frequency() / GDawConfig.sample_rate, 1.0)
#		_update_state()
		to_fill -= 1
		current_count += 1

func _process(_delta):
	if current_count < sample_count:
		_fill_buffer()
	else:
		self.stop()

func _ready() -> void:
	self.stream.mix_rate = 44100.0
	playback = self.get_stream_playback()
	_fill_buffer()
	self.play()
#	_update_envelope()

