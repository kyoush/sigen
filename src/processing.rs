use std::f32::consts::PI;
use std::process::exit;
use std::error::Error;
use hound::WavSpec;
use rtaper::{WindowType, TaperSpec, apply_taper_both, apply_taper_fade_out};

use crate::commands::{gen::WaveFormCommands, common::TaperSpecOptions, taper::TaperOptions};
use crate::fileio::{wavread, wavwrite, gen_file_name};

const CH: u16 = 2; // stereo
const BITS_PER_SAMPLE: u16 = 16;
const AMP_MIN: f64 = 0.0;
const AMP_MAX: f64 = 1.0;

struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: u32,
    pub d: u32,
    pub taper_spec: TaperSpec,
}

fn value_verify<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd + Copy,
{
    if min > max {
        panic!("unexpected error");
    }

    if value < min {
        min
    } else if max < value {
        max
    } else {
        value
    }
}

fn get_taper_spec(opt: &TaperSpecOptions) -> TaperSpec {
    let taper_type = match opt.window_type.as_str() {
        "linear" => { WindowType::Linear }
        "hann" => { WindowType::Hann }
        "cos" => { WindowType::Cosine }
        "blackman" => { WindowType::Blackman }
        _ => { WindowType::Hann }
    };

    TaperSpec {
        taper_type: taper_type,
        taper_length: opt.length_of_taper,
    }
}

pub fn apply_taper_to_wav(options: &TaperOptions) -> Result<(), Box<dyn Error>> {
    let taper_spec = get_taper_spec(&options.taper_opt);
    let (mut samples, spec) = wavread::read_wav_file(options.input.as_str())?;
    let num_ch = samples.len();

    for i in 0..num_ch {
        apply_taper_both(&mut samples[i], &taper_spec)?;
    }

    let fileinfo = crate::fileio::set_output_filename(options.output.clone(), options.input.as_str())?;
    wavwrite::write_wav_file(spec, fileinfo.name.as_str(), &samples, true, true)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}

fn generate_sine_wave(spec: &SignalSpec, frequency: u32) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let t = i as f32 / spec.fs as f32;
        let sample = spec.amp * (2.0 * PI * frequency as f32 * t).sin() as f64;
        samples.push(sample);
    }

    if let Err(e) = apply_taper_both(&mut samples, &spec.taper_spec) {
        eprintln!("failed to apply taper: {}", e);
    }

    samples
}

fn generate_white_noise(spec: &SignalSpec) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = spec.amp * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    if let Err(e) = apply_taper_both(&mut samples, &spec.taper_spec) {
        eprintln!("failed to apply taper: {}", e);
    }

    samples
}

fn generate_linear_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Vec<f64> {
    let sample_count = (spec.d as u32 * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let phase = 2.0 * PI as f64 * (lowfreq * t + ((highfreq - lowfreq) / (2.0 * spec.d as f64)) * t * t);
        samples.push(spec.amp * phase.sin());
    }

    samples
}

fn generate_log_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Vec<f64> {
    let sample_count = (spec.d as u32 * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let freq = lowfreq * ((highfreq / lowfreq).powf(t / spec.d as f64));
        let phase = 2.0 * PI as f64 * freq * t;
        samples.push(spec.amp * phase.sin());
    }

    let cutoff_index = (sample_count as f32 * 0.75) as usize;

    let signal = &samples[0..cutoff_index];
    signal.to_vec()
}

fn generate_tsp_signal(spec: &SignalSpec, tsp_type: String, lowfreq: i32, highfreq: i32) -> Vec<f64> {
    let mut samples;

    if tsp_type == "linear" {
        samples = generate_linear_tsp(&spec, lowfreq as f64, highfreq as f64);
    } else if tsp_type == "log" {
        samples = generate_log_tsp(&spec, lowfreq as f64, highfreq as f64);
    } else {
        eprintln!("Error: unexpected type of tsp signal");
        exit(1);
    }
    if let Err(e) = apply_taper_fade_out(&mut samples, &spec.taper_spec) {
        eprintln!("failed to apply taper: {}", e);
    }

    samples
}

pub fn signal_generator(args: &super::commands::gen::GenOptions) -> Result<(), Box<dyn Error>>{
    let (common_options, taper_options, duration) = match &args.waveform {
        super::commands::gen::WaveFormCommands::Sine(sine_options) => (
            Some(sine_options.options.clone()),
            &sine_options.taper_opt,
            sine_options.duration,
        ),
        super::commands::gen::WaveFormCommands::White(white_options) => (
            Some(white_options.options.clone()),
            &white_options.taper_opt,
            white_options.duration,
        ),
        super::commands::gen::WaveFormCommands::Tsp(tsp_options) => (
            Some(tsp_options.options.clone()),
            &tsp_options.taper_opt,
            tsp_options.duration,
        ),
    };

    let taper_spec = get_taper_spec(taper_options);

    let (signal_spec, enable_l, enable_r, filename_ch) = match common_options {
        Some(ref common_options) =>  {
            let signal_spec = SignalSpec {
                amp: value_verify(common_options.amplitude, AMP_MIN, AMP_MAX),
                ch: common_options.channels.clone(),
                fs: common_options.rate_of_sample,
                d: duration,
                taper_spec: taper_spec,
            };

            match signal_spec.ch.as_str() {
                "L" => (signal_spec, true, false, "l_only"),
                "R" => (signal_spec, false, true, "_r_only"),
                "LR" => (signal_spec, true, true, ""),
                _ => {
                    return Err("unknown spec ch error".into());
                }
            }
        },
        None => {
           return Err("common_options is not set".into());
        }
    };

    // generate signals
    let samples;
    let fileinfo;
    match &args.waveform {
        WaveFormCommands::Sine(sine_options) => {
            let f_verified  = value_verify(sine_options.frequency, 0, signal_spec.fs / 2);

            fileinfo = gen_file_name(
                &common_options.unwrap().output_filename,
                "sine",
                f_verified as i32,
                -1, filename_ch,
                signal_spec.d
            )?;
            samples = generate_sine_wave(&signal_spec, f_verified);
        }
        WaveFormCommands::White(_) => {
            fileinfo = gen_file_name(
                &common_options.unwrap().output_filename,
                "white",
                -1,
                -1,
                filename_ch,
                signal_spec.d
            )?;
            samples = generate_white_noise(&signal_spec);
        }
        WaveFormCommands::Tsp(tsp_options) => {
            let startf_verified = value_verify(tsp_options.startf, 0, (signal_spec.fs / 2) as i32);
            let endf_verified = value_verify(tsp_options.endf, 0, startf_verified);

            fileinfo = gen_file_name(
                &common_options.unwrap().output_filename,
                "tsp",
                startf_verified,
                endf_verified,
                filename_ch,
                signal_spec.d
            )?;
            samples = generate_tsp_signal(&signal_spec, tsp_options.tsp_type.clone(), startf_verified, endf_verified);
        }
    }

    // write wav file
    let wav_spec = WavSpec {
        channels: CH,
        sample_rate: signal_spec.fs,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    let samples_to_write = [samples.clone(), samples];

    wavwrite::write_wav_file(wav_spec, &fileinfo.name, &samples_to_write, enable_l, enable_r)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}
