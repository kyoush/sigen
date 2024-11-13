use std::error::Error;
use std::f32::consts::PI;
use rtaper::{WindowType, TaperSpec};

use crate::commands::common::TaperSpecOptions;

pub struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: i32,
    pub d: f64,
    pub taper_spec: TaperSpec,
}

pub fn get_taper_spec(opt: &TaperSpecOptions) -> TaperSpec {
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

fn trim_end_to_i32(cmd: &str, pattern: &str) -> Result<f64, String> {
    if cmd.to_lowercase().ends_with(pattern) {
        let trim = cmd.trim_end_matches(&pattern);
        trim.parse::<f64>().map_err(|e| e.to_string())
    }else {
        Err("partern not found end".to_string())
    }
}

pub fn parse_duration(duration_cmd: &str) -> Result<f64, Box<dyn Error>> {
    match duration_cmd.parse::<f64>() {
        Ok(val) => { Ok(val) }
        Err(_) => {
            if let Ok(val) = trim_end_to_i32(duration_cmd, "m")          { Ok(val / 1000.0) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "msec")  { Ok(val / 1000.0) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "s")     { Ok(val) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "sec")   { Ok(val) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "min")   { Ok(val * 60.0) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "h")     { Ok(val * 60.0 * 60.0) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "hour")  { Ok(val * 60.0 * 60.0) }
            else if let Ok(val) = trim_end_to_i32(duration_cmd, "hours") { Ok(val * 60.0 * 60.0) }
            else {
                return Err(format!("cannot parse duration [{}]", duration_cmd).into())
            }
        }
    }
}

pub fn generate_sine_wave(spec: &SignalSpec, frequency: i32) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let t = i as f32 / spec.fs as f32;
        let sample = spec.amp * (2.0 * PI * frequency as f32 * t).sin() as f64;
        samples.push(sample);
    }

    rtaper::apply_taper_both(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}

pub fn generate_white_noise(spec: &SignalSpec) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = spec.amp * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    rtaper::apply_taper_both(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}

fn generate_linear_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let phase = 2.0 * PI as f64 * (lowfreq * t + ((highfreq - lowfreq) / (2.0 * spec.d as f64)) * t * t);
        samples.push(spec.amp * phase.sin());
    }

    Ok(samples)
}

fn generate_log_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let freq = lowfreq * ((highfreq / lowfreq).powf(t / spec.d));
        let phase = 2.0 * PI as f64 * freq * t;
        samples.push(spec.amp * phase.sin());
    }

    let cutoff_index = (sample_count as f32 * 0.75) as usize;
    let signal = &samples[0..cutoff_index];

    Ok(signal.to_vec())
}

pub fn generate_tsp_signal(spec: &SignalSpec, tsp_type: String, lowfreq: i32, highfreq: i32) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut samples;

    if tsp_type == "linear" {
        samples = generate_linear_tsp(&spec, lowfreq as f64, highfreq as f64)?;
    } else if tsp_type == "log" {
        samples = generate_log_tsp(&spec, lowfreq as f64, highfreq as f64)?;
    } else {
        return Err("unexpected type of tsp signal".into());
    }

    rtaper::apply_taper_fade_out(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}

pub fn generate_pwm_signal(spec: &SignalSpec, freq: i32, duty: u32) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut samples = vec![0.0; (spec.d * spec.fs as f64) as usize];
    let period_samples = spec.fs / freq;
    let high_samples = (period_samples  as f64 * (duty as f64 / 100.0)) as usize;

    for period_start_point in (0..(spec.d * spec.fs as f64) as i32).step_by(period_samples as usize) {
        let end = period_start_point + high_samples as i32;
        if end > (spec.d * spec.fs as f64) as i32 {
            break;
        }
        for high_point in period_start_point..end {
            samples[high_point as usize] = spec.amp;
        }
    }

    rtaper::apply_taper_both(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}
