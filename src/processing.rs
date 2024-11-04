use std::f32::consts::PI;
use std::process::exit;
use std::error::Error;
use rtaper::TaperSpec;
use crate::commands::taper::TaperOptions;
use crate::SignalSpec;
use crate::fileio::{wavread, wavwrite, FileInfo};

pub fn value_verify<T>(value: T, min: T, max: T) -> T
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

pub fn apply_taper_to_wav(options: &TaperOptions, taper_spec: &TaperSpec) -> Result<FileInfo, Box<dyn Error>> {
    let (mut samples, spec) = wavread::read_wav_file(options.input.as_str())?;
    let num_ch = samples.len();
    for i in 0..num_ch {
        rtaper::apply_taper_both(&mut samples[i], &taper_spec)?;
    }

    let fileinfo = crate::fileio::set_output_filename(options.output.clone(), options.input.as_str())?;
    wavwrite::write_wav_file(spec, fileinfo.name.as_str(), &samples, true, true)?;

    Ok(fileinfo)
}

pub fn generate_sine_wave(spec: &SignalSpec, frequency: u32) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    let f_verified  = value_verify(frequency, 0, spec.fs / 2) as f32;

    for i in 0..sample_count {
        let t = i as f32 / spec.fs as f32;
        let sample = spec.amp * (2.0 * PI * f_verified * t).sin() as f64;
        samples.push(sample);
    }

    if let Err(e) = rtaper::apply_taper_both(&mut samples, &spec.taper_spec) {
        eprintln!("failed to apply taper: {}", e);
    }

    samples
}

pub fn generate_white_noise(spec: &SignalSpec) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = spec.amp * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    if let Err(e) = rtaper::apply_taper_both(&mut samples, &spec.taper_spec) {
        eprintln!("failed to apply taper: {}", e);
    }

    samples
}

fn generate_linear_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Vec<f64> {
    let sample_count = (spec.d as u32 * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    let h_verified = value_verify(highfreq, 0.0, spec.fs as f64 / 2.0);
    let l_verified = value_verify(lowfreq, 0.0, h_verified);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let phase = 2.0 * PI as f64 * (lowfreq * t + ((h_verified - l_verified) / (2.0 * spec.d as f64)) * t * t);
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
    if let Err(e) = rtaper::apply_taper_fade_out(&mut samples, &spec.taper_spec) {
        eprintln!("failed to apply taper: {}", e);
    }

    samples
}
