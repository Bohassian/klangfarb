extends AudioStreamPlayer

# controllable frequency interface
export(float, 20, 8000, 10) var freq = 440.0

# load the GDNative script connected to the rust lib
var SineWave = preload("res://SineWave.gdns")
# make an instance of our one "class" in rust lib
var wave = SineWave.new()

# initialize the Godot stream we fill up with samples
var playback: AudioStreamPlayback = null

func _fill_buffer() -> void:
	# get count of Frames (sample, sample) available in stream
	var to_fill = playback.get_frames_available()
	if to_fill > 0:
		# ask Rust to generate N frames at freq
		# Array<Vector2> gets pushed to the 
		# playback stream buffer
		playback.push_buffer(wave.frames(freq, to_fill)) 

func _process(_delta):
		_fill_buffer()

func _ready() -> void:
	# ensure Godot/Sine have the same sample rate
	wave.set_sample_rate(self.stream.mix_rate)
	# get our AudioStreamPlayback object
	playback = self.get_stream_playback()
	# prefill the stream's sample buffer (which feeds DAC)
	_fill_buffer()
	# start the audio
	self.play()
