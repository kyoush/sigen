# sigen

A tool for generating WAV files of various signal types. \
You can control parameters such as length, frequency, and channels through command-line arguments.

name: signal generator => sig. gen. => sigen

### usage

```
$ ./signal-generator -h
Usage: sigen <COMMAND>

Commands:
  sine   generate a wav file with a sine wave
  white  generate a wave file with a white noise
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

support signal type:
- sine
- white noise

### example

```bash
$ ./sigen sine # using default value
WAV file "sine_440hz_30s.wav" created successfully
```

```bash
$ ./sigen sine -f 5000 -c R
WAV file "sine_5khz_30s_r_only.wav" created successfully
```

```bash
$ ./sigen white -d 600     
WAV file "white_10min.wav" created successfully
```
