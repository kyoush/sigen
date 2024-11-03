use std::f32::consts::PI;
use std::process::exit;

use crate::SignalSpec;
use crate::fileio::{wavwrite, wavread};

pub fn apply_taper_to_wav() {
    samples = read_wave_file;
    rtaper::apply_taper_both();
    fileinfo = 
}

pub fn generate_sine_wave(spec: &SignalSpec, frequency: u32) -> Vec<f64> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let t = i as f32 / spec.fs as f32;
        let sample = spec.amp * (2.0 * PI * frequency as f32 * t).sin() as f64;
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
