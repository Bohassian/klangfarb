extends AudioStreamPlayer

# control wave form
export(String, "sine", "square", "triangle", "sawtooth", "white_noise", "brown_noise") var waveform = "sine"
# controllable frequency interface
export(float, 20, 8000, 5) var freq = 440.0
# toggle between playing a waveform or an instrument
export(bool) var play_instrument = false
# bending the waveform
export(bool) var apply_bend = false
export(Vector2) var phasor_bend = Vector2(0.5, 0.5)
# duration related
export(bool) var continuous = true
# Used for the total duration of an Instrument note
export(int, 0, 30000, 100) var duration = 3000
#Attack/Decay/Release/Sustain
export(int, 0, 5000, 100) var attack = 100
export(int, 0, 5000, 100) var decay = 100
export(int, 0, 5000, 100) var release = 100
export(float, 0.0, 1.0, 0.1) var sustain = 0.5
#Cutoff
export(float, 20, 8000, 5) var cutoff = 6000

# Frequency Modulation
export(bool) var frequency_modulation = false
export(float, 0.0, 50.0, 0.1) var fm_multiplier = 1.0
export(float, 0.0, 1000.0, 1.0) var fm_index = 1.0

# load the GDNative script connected to the rust lib
var MonoSynth = preload("res://MonoSynth.gdns")

# make an instance of our one "class" in rust lib
var synth = MonoSynth.new()

# initialize the Godot stream we fill up with samples
var playback: AudioStreamPlayback = null

func _fill_buffer() -> void:
	# get count of Frames (sample, sample) available in stream
	var to_fill = playback.get_frames_available()
	if to_fill > 0:
		# ask Rust to generate N frames at freq
		# Array<Vector2> gets pushed to the
		# playback stream buffer
		playback.push_buffer(synth.frames(to_fill))

func _check_waveform():
	if waveform == "square":
		synth.square()
	elif waveform == "sine":
		synth.sine()
	elif waveform == "triangle":
		synth.triangle()
	elif waveform == "sawtooth":
		synth.sawtooth()
	elif waveform == "white_noise":
		synth.white_noise()
	elif waveform == "brown_noise":
		synth.brown_noise()

func _process(_delta):
	if self.is_playing():
		synth.continuous(continuous)
		synth.set_attack(attack)
		synth.set_decay(decay)
		synth.set_sustain(sustain)
		synth.set_release(release)
		synth.play_instrument(play_instrument)
		_check_waveform()
		_fill_buffer()

func _ready() -> void:
	# buffer length of 10ms gives us ~realtime response to input changes
	self.stream.buffer_length = 0.05
	# ensure Godot/Sine have the same sample rate
	synth.set_sample_rate(self.stream.mix_rate)
	# get our AudioStreamPlayback object
	playback = self.get_stream_playback()
	# prefill the stream's sample buffer (which feeds DAC)
	_check_waveform()
	_fill_buffer()

func toggle_playback():
	self._set_playing(!self.is_playing())

func _on_duration_value_changed(value):
	synth.duration(int(value))

func _on_pitch_value_changed(value):
	synth.frequency(value)

func _on_bell_click(event):
	if event is InputEventMouseButton && event.is_pressed():
		synth.trigger()
