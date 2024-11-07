use std::f32::consts::PI;
use std::error::Error;
use hound::WavSpec;
use rtaper::{WindowType, TaperSpec};

use crate::commands::{gen, common::TaperSpecOptions, taper::TaperOptions};
use crate::fileio::{wavread, wavwrite, gen_file_name};

const CH: u16 = 2; // stereo
const BITS_PER_SAMPLE: u16 = 16;

pub struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: i32,
    pub d: i32,
    pub taper_spec: TaperSpec,
}

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

pub fn apply_taper_to_wav(options: &TaperOptions) -> Result<(), Box<dyn Error>> {
    let taper_spec = get_taper_spec(&options.taper_opt);
    let (mut samples, spec) = wavread::read_wav_file(options.input.as_str())?;
    let num_ch = samples.len();

    for i in 0..num_ch {
        rtaper::apply_taper_both(&mut samples[i], &taper_spec)?;
    }

    let fileinfo = crate::fileio::set_output_filename(options.output.clone(), options.input.as_str())?;
    wavwrite::write_wav_file(spec, fileinfo.name.as_str(), &samples, true, true)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}

fn generate_sine_wave(spec: &SignalSpec, frequency: i32) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let t = i as f32 / spec.fs as f32;
        let sample = spec.amp * (2.0 * PI * frequency as f32 * t).sin() as f64;
        samples.push(sample);
    }

    rtaper::apply_taper_both(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}

fn generate_white_noise(spec: &SignalSpec) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = spec.amp * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    rtaper::apply_taper_both(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}

fn generate_linear_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let phase = 2.0 * PI as f64 * (lowfreq * t + ((highfreq - lowfreq) / (2.0 * spec.d as f64)) * t * t);
        samples.push(spec.amp * phase.sin());
    }

    Ok(samples)
}

fn generate_log_tsp(spec: &SignalSpec, lowfreq: f64, highfreq: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    let sample_count = (spec.d * spec.fs) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for n in 0..sample_count {
        let t = n as f64 / spec.fs as f64;
        let freq = lowfreq * ((highfreq / lowfreq).powf(t / spec.d as f64));
        let phase = 2.0 * PI as f64 * freq * t;
        samples.push(spec.amp * phase.sin());
    }

    let cutoff_index = (sample_count as f32 * 0.75) as usize;
    let signal = &samples[0..cutoff_index];

    Ok(signal.to_vec())
}

fn generate_tsp_signal(spec: &SignalSpec, tsp_type: String, lowfreq: i32, highfreq: i32) -> Result<Vec<f64>, Box<dyn Error>> {
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

fn generate_pwm_signal(spec: &SignalSpec, freq: i32, duty: u32) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut samples = vec![0.0; (spec.fs * spec.d) as usize];
    let period_samples = spec.fs / freq;
    let high_samples = (period_samples  as f64 * (duty as f64 / 100.0)) as usize;

    for period_start_point in (0..spec.fs * spec.d).step_by(period_samples as usize) {
        let end = period_start_point + high_samples as i32;
        if end > spec.fs * spec.d {
            break;
        }
        for high_point in period_start_point..end {
            samples[high_point as usize] = spec.amp;
        }
    }

    rtaper::apply_taper_both(&mut samples, &spec.taper_spec)?;

    Ok(samples)
}

pub fn signal_generator(args: &gen::GenOptions) -> Result<(), Box<dyn Error>>{
    let common_options = args.waveform.get_common_opt();
    let taper_spec = args.waveform.get_taper_spec();
    let signal_spec = common_options.get_signal_spec(taper_spec);

    let (enable_l, enable_r, filename_ch) = match signal_spec.ch.as_str() {
        "L" => (true, false, "_l_only"),
        "R" => (false, true, "_r_only"),
        "LR" => (true, true, ""),
        _ => {
            return Err("unknown spec ch error".into());
        }
    };

    let (sig_type, startf, endf) = args.waveform.get_fileinfo(signal_spec.fs);

    let fileinfo = gen_file_name(
        &common_options.output_filename,
        sig_type,
        startf,
        endf,
        filename_ch,
        signal_spec.d,
    )?;

    // generate signals
    let samples;
    match &args.waveform {
        gen::WaveFormCommands::Sine(_) => {
            samples = generate_sine_wave(&signal_spec, startf)?;
        }
        gen::WaveFormCommands::White(_) => {
            samples = generate_white_noise(&signal_spec)?;
        }
        gen::WaveFormCommands::Tsp(tsp_options) => {
            samples = generate_tsp_signal(&signal_spec, tsp_options.tsp_type.clone(), startf, endf)?;
        }
        gen::WaveFormCommands::Pwm(pwm_options) => {
            let d_verified = value_verify(pwm_options.percent_of_duty, 0, 100);
            samples = generate_pwm_signal(&signal_spec, startf, d_verified)?;
        }
    }

    // write wav file
    let wav_spec = WavSpec {
        channels: CH,
        sample_rate: signal_spec.fs as u32,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    let samples_to_write = [samples.clone(), samples];

    wavwrite::write_wav_file(wav_spec, &fileinfo.name, &samples_to_write, enable_l, enable_r)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}
