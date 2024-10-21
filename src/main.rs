use std::env;
use hound::{WavSpec,WavWriter};
use std::process::exit;
use std::f32::consts::PI;

const FS: u32 = 44100; // Hz
const CH: u32 = 2; // stereo
const AMP: f32 = 0.45;
const D: u32 = 30; // sec
const N: usize = 2048; // samples of taper

fn generate_sine_wave(amplitude: f32, duration: u32, frequency: u32) -> Vec<f32> {
    let sample_count = (duration * FS) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let t = i as f32 / FS as f32; // 現在のサンプルの時間
        let sample = amplitude * (2.0 * PI * frequency as f32 * t).sin();
        // サンプルをクリップして i16 に変換
        samples.push(sample);
    }

    samples
}

fn generate_white_noise(amplitude: f32, duration: u32) -> Vec<f32> {
    let sample_count = (duration * FS) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    
    for _ in 0..sample_count {
        let noise = amplitude * (rand::random::<f32>() * 2.0 - 1.0);
        samples.push(noise);
    }

    samples
}

fn apply_linear_taper(samples: &mut [f32], taper_size: usize) {
    let total_samples = samples.len();
    
    let taper_size = taper_size.min(total_samples / 2);

    for i in 0..taper_size {
        let factor = i as f32 / taper_size as f32;
        samples[i] *= factor;
    }

    for i in 0..taper_size {
        let factor = 1.0 - (taper_size - i) as f32 / taper_size as f32;
        samples[total_samples - 1 - i] *= factor;
    }
}

fn write_wav_file(filename: &str, samples: &[f32], channels: u32, enable_l: bool, enable_r: bool) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: channels as u16,
        sample_rate: FS,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;

    let total_samples = samples.len();

    for i in 0..total_samples {
        let sample_value: i16 = (samples[i] * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        writer.write_sample(sample_value * (enable_l as i16))?;
        writer.write_sample(sample_value * (enable_r as i16))?;
    }

    writer.flush()?;
    Ok(())
}

fn print_help() {
    println!("Usage: signal_generator -a <amplitude> -d <duration> -t <type> [-f <frequency>]");
    println!("Options:");
    println!("  -a <amplitude>   Amplitude of the signal (default: 0.45)");
    println!("  -d <duration>    Duration of the signal in seconds (default: 30");
    println!("  -t <type>        Type of the signal: 'sine' or 'white' (default: 'sine')");
    println!("  -f <frequency>   Frequency of the sine wave in Hz (default: 440, required if type is 'sine')");
    println!("  -c <channels>    Which channel generate ... [L, R, LR] (default: LR)");
    println!("  -h               Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut amp = AMP;
    let mut d = D;
    let mut signal_type = "sine".to_string();
    let mut freq = 440;
    let mut enable_l = true;
    let mut enable_r = true;
    let mut filename_ch = "";

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-a" => {
                if i + 1 < args.len() {
                    amp = args[i + 1].parse().expect("Invalid amplitude");
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for -a");
                    exit(1);
                }
            }
            "-d" => {
                if i + 1 < args.len() {
                    d = args[i + 1].parse().expect("Invalid duration");
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for -d");
                    exit(1);
                }
            }
            "-t" => {
                if i + 1 < args.len() {
                    signal_type = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for -t");
                    exit(1);
                }
            }
            "-f" => {
                if i + 1 < args.len() {
                    freq = args[i + 1].parse().expect("Invalid frequency");
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for -f");
                    exit(1);
                }
            }
            "-c" => {
                if i + 1 < args.len() {
                    let channel = &args[i + 1];
                    match channel.as_str() {
                        "L" => {
                            enable_l = true;
                            enable_r = false;
                            filename_ch = "_l_only";
                            i += 1;
                        }    // Lチャンネルのみ
                        "R" => {
                            enable_l = false;
                            enable_r = true;
                            filename_ch = "_r_only";
                            i += 1;
                        }    // Rチャンネルのみ
                        "LR" => {
                            enable_l = true;
                            enable_r = true;
                            i += 1;
                        }    // 両チャンネル
                        _ => {
                            eprintln!("Invalid channel option: {}", channel);
                            eprintln!("Valid options are: L, R, LR");
                            exit(1);
                        }
                    };
                } else {
                    eprintln!("Error: Missing value for -c");
                    exit(1);
                }
            }
            "-h" => {
                print_help();
                exit(0);
            }
            _ => {
                eprintln!("Error: Unknown option {}", args[i]);
                exit(1);
            }
        }
        i += 1;
    }

    // Generate samples
    let ch = CH;
    let mut samples = match signal_type.as_str() {
        "sine" => generate_sine_wave(amp, d, freq),
        "white" => generate_white_noise(amp, d),
        _ => {
            eprintln!("Invalid type. Use 'sine' or 'white'");
            exit(1);
        }
    };

    apply_linear_taper(&mut samples, N);

    let filename = format!("{}{}.wav", signal_type, filename_ch);
    if let Err(e) = write_wav_file(&filename, &samples, ch, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file {} created successfully", filename);
    exit(0);
}
