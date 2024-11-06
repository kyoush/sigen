# sigen

![](https://img.shields.io/github/repo-size/kyoush/sigen)
![](https://img.shields.io/github/languages/code-size/kyoush/sigen)
![](https://img.shields.io/github/v/release/kyoush/sigen)
![](https://img.shields.io/github/license/kyoush/sigen)

A tool for generating WAV files of various signal types. \
You can control parameters such as length, frequency, and channels through command-line arguments.

name: signal generator => sig. gen. => sigen

## Usage: sigen \<COMMAND\>

### generate wav file

```
$ sigen gen -h
generate a wav file

Usage: sigen gen <COMMAND>

Commands:
  sine   generate a wav file with a sine wave
  white  generate a wav file with a white noise
  tsp    generate a wav file with a TSP [Time Stretched Pulse] waveform
  pwm    generate a wav file with a PWM (pulse train)
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

- support waveform
  - sine wave
  - white noise
  - tsp: time-stretched pulse
  - PWM (pulse train)

```
sigen taper -h
apply taper processing on existing wav file

Usage: sigen taper [OPTIONS] <INPUT>

Arguments:
  <INPUT>  input filename

Options:
  -o, --output [<OUTPUT>]
          Output filename. If specified without an argument, input file will be overridden
  -l, --length-of-taper <LENGTH_OF_TAPER>
          length of taper set this to zero to disable tapering [default: 4096]
  -w, --window-type <WINDOW_TYPE>
          type of taper [default: linear] [possible values: linear, hann, cos, blackman]
  -h, --help
          Print help
```

### apply taper to wav

- applying tapering to the existing wav file

```
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
$ sigen gen sine
WAV file [sine_440hz_5s.wav] created successfully
```

```bash
# generate R-ch only 5kHz sine wave signal
$ sigen gen sine -f 5000 -c R
WAV file [sine_5khz_5s_r_only.wav] created successfully
```

```bash
# generate 10 min. white noise signal
$ sigen gen white -d 600
WAV file [white_10min.wav] created successfully
```

```bash
# generate 1 sec Log-TSP signal 500Hz to 5kHz without taper
$ sigen gen tsp -t log -s 500 -e 5000 -a 1 -l 0 -d 1
WAV file [tsp_500hz_to_500hz_1s.wav] created successfully
```

### tapering

```bash
# tapering to sine_440hz_30.wav
$ /sigen taper sine_440hz_5s.wav
WAV file [sine_440hz_5s_tapered.wav] created successfully
```

```bash
# you can override input file
$ sigen taper sine_440hz_5s.wav -o
Do you want to overwrite [sine_440hz_5s.wav]? [y/N] y
WAV file [sine_440hz_5s.wav] created successfully (file override)
```

```bash
# you can specify output filename
$ sigen taper sine_440hz_5s.wav -o output.wav
WAV file [output.wav] created successfully
```

## License
This project is licensed under the terms of the GNU General Public License, version 2 (GPL-2.0).  
See the [LICENSE](./LICENSE) file for details.
