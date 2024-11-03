# sigen

![](https://img.shields.io/github/repo-size/kyoush/sigen)
![](https://img.shields.io/github/languages/code-size/kyoush/sigen)

A tool for generating WAV files of various signal types. \
You can control parameters such as length, frequency, and channels through command-line arguments.

name: signal generator => sig. gen. => sigen

## usage

```
$ ./sigen -h
A tool for generating WAV files of various signal types

Usage: sigen <COMMAND>

Commands:
  sine   generate a wav file with a sine wave
  white  generate a wave file with a white noise
  tsp    generate a wave file with a TSP [Time Stretched Pulse] waveform
  taper  apply taper processing on existing wav file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

support signal type:
- sine
- white noise

### generate wav files

- support waveform
  - sine wave
  - white noise
  - tsp: time-stretched pulse
  - PWM/pulse train (future)

```bash
❯ ./sigen sine -h
generate a wav file with a sine wave

Usage: sigen sine [OPTIONS]

Options:
  -f, --frequency <FREQUENCY>
          Frequency of the sine wave in Hz [default: 440]
  -d, --duration <DURATION>
          duration of the signal in seconds [default: 30]
  -a, --amplitude <AMPLITUDE>
          the maximum absolute value of the signal samplitude [default: 0.45]
  -c, --channels <CHANNELS>
          Which channel generate [default: LR] [possible values: L, R, LR]
  -r, --rate-of-sample <RATE_OF_SAMPLE>
          [default: 44100]
  -l, --length-of-taper <LENGTH_OF_TAPER>
          length of taper set this to zero to disable tapering [default: 4096]
  -w, --window-type <WINDOW_TYPE>
          [default: linear] [possible values: linear, hann, cos, blackman]
  -h, --help
          Print help
```

### apply taper to wav

- applying tapering to the existing wav file

```bash
./sigen taper -h
apply taper processing on existing wav file

Usage: sigen taper [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  the input filename

Options:
  -l, --length-of-taper <LENGTH_OF_TAPER>
          length of taper set this to zero to disable tapering [default: 4096]
  -w, --window-type <WINDOW_TYPE>
          [default: linear] [possible values: linear, hann, cos, blackman]
  -h, --help
          Print help
```

## example

### generate

```bash
# generate sine wave using default value
$ sigen sine
WAV file "sine_440hz_30s.wav" created successfully
```

```bash
# generate R-ch only 5kHz sine wave signal
$ sigen sine -f 5000 -c R 
WAV file "sine_5khz_30s_r_only.wav" created successfully
```

```bash
# generate 10 min. white noise signal
$ sigen white -d 600
WAV file "white_10min.wav" created successfully
```



```bash
# generate 1 sec Log-TSP signal 500Hz to 5kHz without taper
$ sigen tsp -t log -s 500 -e 5000 -a 1 -l 0 -d 1 
WAV file "tsp_500hz_to_5khz_1s.wav" created successfully
```

### tapering

```bash
# tapering to sine_440hz_30.wav
$ sigen taper sine_440hz_30s.wav
```
