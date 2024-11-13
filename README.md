# sigen

![](https://img.shields.io/github/repo-size/kyoush/sigen)
![](https://img.shields.io/github/languages/code-size/kyoush/sigen)
![](https://img.shields.io/github/v/release/kyoush/sigen)
![](https://img.shields.io/github/license/kyoush/sigen)

A tool for generating WAV files of various signal types. \
You can control parameters such as length, frequency, and channels through command-line arguments.

name: signal generator => sig. gen. => sigen

## Usage

```
$ sigen help
A tool for generating WAV files of various signal types

Usage: sigen <COMMAND>

Commands:
  gen    generate a wav file
  taper  apply taper processing on existing wav file
  wav    concatenates multiple WAV files into a single file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## generate wav file

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
$ sigen gen sine -h
generate a wav file with a sine wave

Usage: sigen gen sine [OPTIONS]

Options:
  -f, --frequency <FREQUENCY>
          Frequency of the sine wave in Hz [default: 440]
  -a, --amplitude <AMPLITUDE>
          the maximum absolute value of the signal samplitude [default: 0.45]
  -c, --channels <CHANNELS>
          Which channel generate [default: LR] [possible values: L, R, LR]
  -r, --rate-of-sample <RATE_OF_SAMPLE>
          Sample Rate of signal [default: 44100]
  -o, --output-filename <OUTPUT_FILENAME>
          Output Filename
  -l, --length-of-taper <LENGTH_OF_TAPER>
          length of taper set this to zero to disable tapering [default: 4096]
  -w, --window-type <WINDOW_TYPE>
          type of taper [default: linear] [possible values: linear, hann, cos, blackman]
  -d, --duration <DURATION>
          duration of the signal in seconds [default: 5]
  -h, --help
          Print help
```

### examples

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

## apply taper to wav

```
$ sigen taper -h
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

### examples

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

## concatnate multiple wav files

```
$ sigen wav <INPUTS> cat [CAT_COMMANDS] output <OUTPUT>
```

- If CAT_COMMANDS is omitted, the input files will be concatenated as they are.
- It has command-line options very similar to the PDF merging tool [pdftk](https://www.pdflabs.com/docs/pdftk-man-page/).

```bash
# just concatenated multiple files
$ sigen wav input1.wav input2.wav cat output out.wav
```

```bash
# Concatenate the A and B WAV files with a 1ms interval in between.
$ sigen wav A=input1.wav B=input2.wav cat A 100msec B output out.wav
# The following shorthand notation produces the same result as above.
$ sigen wav input1.wav input2.wav cat 100msec output out.wav
```

```bash
$ sigen wav A=input1.wav B=input2.wav C=input3.wav cat A 1s B 2s C output out.wav
# The following shorthand notation produces the same result as above.
$ sigen wav input1.wav input2.wav input3.wav cat 1s 2s output out.wav
```

```bash
# The following complex operation cannot use shorthand notation.
$ sigen wav A=input1.wav B=input2.wav C=input3.wav cat A 100msec B 50msec A 100msec B 1s C output out.wav
```

## License
This project is licensed under the terms of the GNU General Public License, version 2 (GPL-2.0).  
See the [LICENSE](./LICENSE) file for details.
