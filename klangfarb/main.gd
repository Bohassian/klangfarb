extends AudioStreamPlayer

# control wave form
export(String, "sine", "square", "triangle", "sawtooth") var waveform = "sine"
# controllable frequency interface
export(float, 20, 8000, 5) var freq = 440.0
# bending the waveform
export(bool) var apply_bend = false
export(Vector2) var phasor_bend = Vector2(0.5, 0.5)
# duration related
export(bool) var continuous = true
export(int, 0, 5000, 100) var duration = 3000
#Attack/Decay/Release/Sustain
export(int, 0, 5000, 100) var attack = 100
export(int, 0, 5000, 100) var decay = 100
export(int, 0, 5000, 100) var release = 100
export(float, 0.0, 1.0, 0.1) var sustain = 0.5
#Cutoff
export(float, 20, 8000, 5) var cutoff = 6000

# load the GDNative script connected to the rust lib
var MonoSynth = preload("res://MonoSynth.gdns")

# make an instance of our one "class" in rust lib
var wave = MonoSynth.new()

# initialize the Godot stream we fill up with samples
var playback: AudioStreamPlayback = null

func _fill_buffer() -> void:
	# get count of Frames (sample, sample) available in stream
	var to_fill = playback.get_frames_available()
	if to_fill > 0:
		# ask Rust to generate N frames at freq
		# Array<Vector2> gets pushed to the 
		# playback stream buffer
		playback.push_buffer(wave.frames(to_fill)) 

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
		wave.apply_bend(apply_bend)		
		wave.frequency(freq)
		wave.phasor_bend(phasor_bend)
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
#		freq = event.position.x
		phasor_bend.x = event.position.x / 1024
		phasor_bend.y = event.position.y / 600
