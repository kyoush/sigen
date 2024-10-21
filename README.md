# signal-generator

A tool for generating WAV files of various signal types. \
You can control parameters such as length, frequency, and channels through command-line arguments.

### usage

```
$ ./signal-generator -h
Usage: signal_generator -a <amplitude> -d <duration> -t <type> [-f <frequency>]
Options:
  -a <amplitude>   Amplitude of the signal (default: 0.45)
  -d <duration>    Duration of the signal in seconds (default: 30)
  -t <type>        Type of the signal: 'sine' or 'white' (default: 'sine')
  -f <frequency>   Frequency of the sine wave in Hz (default: 440, required if type is 'sine')
  -c <channels>    Which channel generate ... [L, R, LR] (default: LR)
  -h               Show this help message
```

support signal type:
- sine
- white noise

### example

```bash
./signal-generator # using default value
```

=> `sine_440hz_30s.wav` has generated!

```bash
./signal-generator -f 5000 -c R
```

=> `sine_5khz_30s_r_only.wav` has generated!

```bash
./signal-generator -t white -d 600
```

=> `white_10min.wav` has generated!
