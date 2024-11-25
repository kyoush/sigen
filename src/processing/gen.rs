use std::{collections::VecDeque, error::Error};
use std::f64::consts::PI;
use rustfft::{ FftPlanner, num_complex::Complex, num_traits::Zero};
use rtaper::{WindowType, TaperSpec};

use crate::commands::common::TaperSpecOptions;

pub struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: i32,
    pub d: f64,
    pub taper_spec: Option<TaperSpec>,
}

pub fn get_taper_spec(opt: Option<&TaperSpecOptions>) -> Option<TaperSpec> {
    match opt {
        Some(opt) => {
            let taper_type = match opt.window_type.as_str() {
                "linear" => { WindowType::Linear }
                "hann" => { WindowType::Hann }
                "cos" => { WindowType::Cosine }
                "blackman" => { WindowType::Blackman }
                _ => { WindowType::Linear }
            };
        
            Some(TaperSpec {
                taper_type: taper_type,
                taper_length: opt.length_of_taper,
            })
        }
        None => { None }
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

pub fn parse_freq(freq_cmd: &str) -> Result<i32, Box<dyn Error>> {
    match freq_cmd.parse::<i32>() {
        Ok(val) => { Ok (val) }
        Err(_) => {
            if let Ok(val) = trim_end_to_i32(freq_cmd, "k") { Ok(val as i32 * 1000) }
            else {
                return Err(format!("cannot parse frequency [{}]", freq_cmd).into())
            }
        }
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

fn do_apply_taper_end(samples: &mut Vec<f64>, taper_spec: &Option<TaperSpec>,
) -> Result<Vec<f64>, Box<dyn Error>> {
    match taper_spec {
        Some(spec) => {
            rtaper::apply_taper_fade_out(samples, &spec)?;
            Ok(samples.to_vec())
        }
        None => {
            return Err("taper spec is not set".into());
        }
    }
}

fn do_apply_taper_both(samples: &mut Vec<f64>, taper_spec: &Option<TaperSpec>) -> Result<Vec<f64>, Box<dyn Error>>{
    match taper_spec {
        Some(spec) => {
            rtaper::apply_taper_both(samples, &spec)?;
            Ok(samples.to_vec())
        }
        None => {
            return Err("taper spec is not set".into())
        }
    }
}

pub fn generate_sine_wave(spec: &SignalSpec, freq: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let t = i as f64/ spec.fs as f64;
        let sample = spec.amp * (2.0 * PI * freq * t).sin();
        samples.push(sample);
    }

    do_apply_taper_both(&mut samples, &spec.taper_spec)?;
    Ok(samples)
}

fn generate_white_noise(spec: &SignalSpec) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = spec.amp * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    Ok(samples)
}

fn generate_pink_noise(spec: &SignalSpec) -> Result<Vec<f64>, Box<dyn Error>> {
    let white_noise = generate_white_noise(spec)?
        .into_iter()
        .enumerate()
        .map(|(_, real)| {
            let c = Complex::new(real, 0.0);
            c
        })
        .collect::<Vec<_>>();
    let sample_count = white_noise.len();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(sample_count);
    let mut spectrum = white_noise.clone();
    fft.process(&mut spectrum);

    for (i, freq) in spectrum.iter_mut().enumerate() {
        if i == 0 {
            continue;
        }
        let f = i as f64;
        let scaling_factor = 1.0 / f.sqrt();
        freq.re *= scaling_factor;
        freq.im *= scaling_factor;
    }

    let ifft = planner.plan_fft_inverse(sample_count);
    ifft.process(&mut spectrum);

    let max_value = spectrum.iter()
        .map(|c| (c.re.powi(2) + c.im.powi(2)).sqrt())
        .fold(0.0, f64::max);

    let output = spectrum.iter()
        .map(|c| spec.amp * c.re / max_value)
        .collect::<Vec<f64>>();

    Ok(output)
}

pub fn generate_noise(spec: &SignalSpec, noise_type: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut samples = match noise_type {
        "white" => { generate_white_noise(spec) }?,
        "pink" => { generate_pink_noise(spec) }?,
        &_ => { return Err("unknown noise type".into()) }
    };

    do_apply_taper_both(&mut samples, &spec.taper_spec)?;
    Ok(samples)
}

fn design_linear_tsp_spectrum(n: usize, flip_sw: f64) -> Vec<Complex<f64>> {
    let j = n / 2;
    let mut up_tsp_complex: Vec<Complex<f64>> = vec![Complex::zero(); n];

    for k in 0..=j {
        let freq_ratio = ((k as f64) / (n as f64)).powi(2);
        let angle = flip_sw * 2.0 * PI * (j as f64) * freq_ratio;
        let complex_angle = Complex::new(0.0, angle);

        up_tsp_complex[k] = complex_angle.exp();
    }

    for k in (1..j).rev() {
        up_tsp_complex[n - k] = up_tsp_complex[k].conj();
    }

    up_tsp_complex
}

// @ref https://www.kanedayyy.jp/asp/ASP/IRseminor2016.pdf
fn design_log_tsp_spectrum(n: usize, flip_sw: f64) -> Vec<Complex<f64>> {
    let j = n / 2;
    let mut up_tsp_complex: Vec<Complex<f64>> = vec![Complex::zero(); n];

    up_tsp_complex[0] = Complex::new(1.0, 0.0);

    let a = (j as f64 * PI) / ((n as f64 / 2.0) * (n as f64 / 2.0).ln());

    for k in 1..=j {
        let angle = flip_sw * a * k as f64 * (k as f64).ln();
        let complex_angle = Complex::new(0.0, angle);

        up_tsp_complex[k] = (1.0 / (k as f64).sqrt()) * complex_angle.exp();
    }

    for k in (1..j).rev() {
        up_tsp_complex[n - k] = up_tsp_complex[k].conj();
    }

    up_tsp_complex
}

fn exec_generate_tsp(
    spec: &SignalSpec,
    design_tsp_spect: fn(usize, f64) -> Vec<Complex<f64>>,
    enable_flip: bool,
) -> Result<Vec<f64>, Box<dyn Error>> {
    let n_samples = spec.d * spec.fs as f64;
    let pow = (n_samples * 2.0).log2().ceil() as i32;
    let n = 1 << pow;
    let flip_sw = if enable_flip { 1.0 } else { -1.0 };

    let mut up_tsp_real: Vec<f64> = vec![0.0; n];
    let mut up_tsp_complex = design_tsp_spect(n, flip_sw);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(n);
    fft.process(&mut up_tsp_complex);

    for (i, c) in up_tsp_complex.iter().enumerate() {
        up_tsp_real[i] = c.re / (n as f64);
    }

    let max_value = up_tsp_real
        .iter()
        .map(|x| x.abs())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let output = up_tsp_real.iter()
        .map(|x| spec.amp * x / max_value)
        .collect::<Vec<f64>>();

    let shift = ((3.0 / 4.0) * n as f64).floor() as usize;
    let mut deque :VecDeque<f64> = VecDeque::from(output);
    if enable_flip {
        deque.rotate_right(shift);
    }else {
        deque.rotate_left(shift);
    }


    Ok(deque.into())
}

pub fn generate_tsp_signal(spec: &SignalSpec, tsp_type: &str, enable_flip: bool) -> Result<Vec<f64>, Box<dyn Error>> {
    let output = match tsp_type {
        "linear" => { exec_generate_tsp(&spec, design_linear_tsp_spectrum, enable_flip)? }
        "log" => { exec_generate_tsp(&spec, design_log_tsp_spectrum, enable_flip)? }
        _ => { return Err("unexpected type of tsp signal".into()); }
    };

    Ok(output)
}

fn generate_log_sweep_signal(spec: &SignalSpec, s: f64, e: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count  = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    let ln_ratio = (e / s).ln();
    let k = ln_ratio / spec.d;

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let phase = 2.0 * PI * s * ((k * t).exp() - 1.0) / k;
        samples.push(spec.amp * phase.sin());
    }

    Ok(samples)
}

fn generate_linear_sweep_signal(spec: &SignalSpec, s: f64, e: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs as f64) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let phase = 2.0 * PI * (s * t + ((e - s) / (2.0 * spec.d as f64)) * t * t);
        samples.push(spec.amp * phase.sin())
    }

    Ok(samples)
}

pub fn generate_sweep_signal(
    spec: &SignalSpec,
    sweep_type: &str,
    s: i32,
    e: i32,
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut output = match sweep_type {
        "linear" => { generate_linear_sweep_signal(spec, s as f64, e as f64)? }
        "log" => { generate_log_sweep_signal(spec, s as f64, e as f64)? }
        _=> { return Err("Unknown swept type".into()) }
    };

    do_apply_taper_end(&mut output, &spec.taper_spec)?;
    Ok(output)
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

    do_apply_taper_both(&mut samples, &spec.taper_spec)?;
    Ok(samples)
}

pub fn generate_zeros(spec: &SignalSpec) -> Result<Vec<f64>, Box<dyn Error>> {
    Ok(vec![0.0; (spec.d * spec.fs as f64) as usize])
}
