extends AudioStreamPlayer

# controllable frequency interface
export(float, 20, 8000, 5) var freq = 440.0
# control wave form
export(String, "sine", "square", "triangle", "sawtooth") var waveform = "sine"

# load the GDNative script connected to the rust lib
var Osc = preload("res://Osc.gdns")

# make an instance of our one "class" in rust lib
var wave = Osc.new()

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

func _check_waveform():
	if waveform == "square":
		wave.square()
	elif waveform == "sine":
		wave.sine()
	elif waveform == "triangle":
		wave.triangle()
	elif waveform == "sawtooth":
		wave.sawtooth()

func _process(_delta):
	if self.is_playing():
		_check_waveform()
		_fill_buffer()

func _ready() -> void:
	# buffer length of 100ms gives us ~realtime response to input changes
	self.stream.buffer_length = 0.1
	# ensure Godot/Sine have the same sample rate
	wave.set_sample_rate(self.stream.mix_rate)
	# get our AudioStreamPlayback object
	playback = self.get_stream_playback()
	# prefill the stream's sample buffer (which feeds DAC)
	_check_waveform()
	_fill_buffer()

func _input(event):
	# Mouse in viewport coordinates.
	if event is InputEventMouseButton:
		print("Mouse Click/Unclick at: ", event.position)
	elif event is InputEventMouseMotion:
		freq = event.position.x
		print("Mouse Motion at: ", event.position)
