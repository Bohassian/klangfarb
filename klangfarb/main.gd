extends AudioStreamPlayer
var playback: AudioStreamPlayback = null

var SineWave = preload("res://SineWave.gdns")
var buffer = SineWave.new()
var duration = 3
var samples = buffer.frames(440.0, 44100.0, duration)
var sample_count = samples.size()
var current_count = 0

func _fill_buffer() -> void:
	var to_fill = playback.get_frames_available()
	
	while to_fill > 0 && current_count != sample_count:
		var sample_index = current_count
		playback.push_frame(Vector2.ONE * samples[sample_index]) # Audio frames are stereo.
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
