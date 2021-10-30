extends AudioStreamPlayer
var playback: AudioStreamPlayback = null

var SineWave = preload("res://SineWave.gdns")
var wave = SineWave.new()
var freq = 440.0

func _fill_buffer() -> void:
	var to_fill = playback.get_frames_available()
	print(to_fill)
	if to_fill > 0:
		playback.push_buffer(wave.frames(freq, to_fill)) 

func _process(_delta):
	_fill_buffer()

func _ready() -> void:
	self.stream.mix_rate = freq * 256
	wave.set_sample_rate(freq * 256)
	playback = self.get_stream_playback()
	_fill_buffer()
	self.play()
