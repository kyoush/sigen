use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use std::path::Path;
use std::io::{self, Write};
use std::process::exit;
use rtaper;

pub struct FileInfo {
    pub name: String,
    pub exists_msg: String,
}

pub struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: u32,
    pub d: u32,
    pub taper_type: String,
    pub taper_len: usize,
}

    let window: Vec<f64> = (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64).cos()))
        .collect();

    // Hanning窓の右半分を信号の後半に適用
    for i in 0..(n / 2) {
        samples[samples.len() - (n / 2) + i] *= window[i + n / 2];
    }
}

pub fn generate_sine_wave(spec: &SignalSpec, frequency: u32) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let t = i as f32 / spec.fs as f32; // 現在のサンプルの時間
        let sample = spec.amp * (2.0 * PI * frequency as f32 * t).sin() as f64;
        // サンプルをクリップして i16 に変換
        samples.push(sample);
    }

    rtaper::apply_taper(&mut samples, &spec);
    samples
}

pub fn generate_white_noise(spec: &SignalSpec) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = spec.amp * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    rtaper::apply_taper(&mut samples, &spec);
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
    let mut signal_vec: Vec<f64> = signal.to_vec();
    signal.to_vec()
}

pub fn generate_tsp_signal(spec: &SignalSpec, tsp_type: String, lowfreq: i32, highfreq: i32) -> Vec<f64> {
    let mut samples;

    if tsp_type == "linear" {
        samples = generate_linear_tsp(&spec, lowfreq as f64, highfreq as f64);
    } else if tsp_type == "log" {
        samples = generate_log_tsp(&spec, lowfreq as f64, highfreq as f64);
    } else {
        eprintln!("Error: unexpected type of tsp signal");
        exit(1);
    }
    rtaper::apply_hanning_fade_out(&mut samples, spec.taper_len);
    samples
}

pub fn write_wav_file(
    spec: &SignalSpec,
    filename: &str,
    samples: &[f64],
    channels: u32,
    enable_l: bool,
    enable_r: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: channels as u16,
        sample_rate: spec.fs,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;

    let total_samples = samples.len();

    for i in 0..total_samples {
        let sample_value: i16 =
            (samples[i] * i16::MAX as f64).clamp(i16::MIN as f64, i16::MAX as f64) as i16;
        writer.write_sample(sample_value * (enable_l as i16))?;
        writer.write_sample(sample_value * (enable_r as i16))?;
    }

    writer.flush()?;
    Ok(())
}

fn file_exists_errorcheck(filename: &String) -> String {
    let mut override_msg = String::new();
    if Path::new(&filename).exists() {
        print!("file \"{}\" is already exists, override? [y/N] ", filename);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error: reading input. Exiting...");
            exit(1);
        }

        match input.trim().to_lowercase().as_str() {
            "n" | "no" | "" => {
                eprintln!("Abort! The operation was canceled by the user.");
                eprintln!("No output was generated.");
                exit(1);
            }
            _ => {
                override_msg = " (file override)".to_string();
            }
        }
    }

    override_msg
}

fn freq_format(freq: i32, prefix: &str) -> String {
    if freq < 0 {
        return String::new();
    }
    
    if freq < 1000 {
        format!("_{}{}hz", prefix, freq)
    } else {
        format!("_{}{}khz", prefix, freq / 1000)
    }
}

fn seconds_format(sec: u32) -> String{
    if sec >= 60 {
        if sec % 60 == 0 {
            format!("_{}min", sec / 60)
        } else {
            format!("_{}min{}s", sec / 60, sec % 60)
        }
    } else {
        format!("_{}s", sec)
    }
}

pub fn gen_file_name(sig_type: &str, start_freq: i32, end_freq: i32, filename_ch: &str, d: u32) -> FileInfo {
    let filename_start_freq = freq_format(start_freq, "");
    let filename_end_freq = freq_format(end_freq, "to_");
    let filename_duration = seconds_format(d);

    let filename = format!(
        "{}{}{}{}{}.wav",
        sig_type, filename_start_freq, filename_end_freq, filename_duration, filename_ch
    );

    let override_msg = file_exists_errorcheck(&filename);

    FileInfo {
        name: filename,
        exists_msg: override_msg,
    }
}
