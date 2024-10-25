use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use std::path::Path;
use std::io::{self, Write};
use std::process::exit;

const FS: u32 = 44_100; // Hz

pub struct FileInfo {
    pub name: String,
    pub exists_msg: String,
}

pub fn generate_sine_wave(amplitude: f64, duration: u32, frequency: u32) -> Vec<f64> {
    let sample_count = (duration * FS) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let t = i as f32 / FS as f32; // 現在のサンプルの時間
        let sample = amplitude * (2.0 * PI * frequency as f32 * t).sin() as f64;
        // サンプルをクリップして i16 に変換
        samples.push(sample);
    }

    samples
}

pub fn generate_white_noise(amplitude: f64, duration: u32) -> Vec<f64> {
    let sample_count = (duration * FS) as usize;
    let mut samples = Vec::with_capacity(sample_count);

    for _ in 0..sample_count {
        let noise = amplitude * (rand::random::<f64>() * 2.0 - 1.0);
        samples.push(noise);
    }

    samples
}

pub fn apply_linear_taper(samples: &mut [f64], taper_size: usize) {
    let total_samples = samples.len();

    let taper_size = taper_size.min(total_samples / 2);

    for i in 0..taper_size {
        let factor = i as f64 / taper_size as f64;
        samples[i] *= factor;
    }

    for i in 0..taper_size {
        let factor = 1.0 - (taper_size - i) as f64 / taper_size as f64;
        samples[total_samples - 1 - i] *= factor;
    }
}

pub fn write_wav_file(
    filename: &str,
    samples: &[f64],
    channels: u32,
    enable_l: bool,
    enable_r: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: channels as u16,
        sample_rate: FS,
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

pub fn gen_file_name(sig_type: &str, freq: i32, filename_ch: &str, d: u32) -> FileInfo {
    let filename_freq = if freq < 0 {
        format!("")
    } else if 0 < freq && freq < 1000 {
        format!("_{}hz", freq)
    } else {
        format!("_{}khz", freq / 1000)
    };

    let filename_duration = if d >= 60 {
        if d % 60 == 0 {
            format!("_{}min", d / 60)
        } else {
            format!("_{}min{}s", d / 60, d % 60)
        }
    } else {
        format!("_{}s", d)
    };

    let filename = format!(
        "{}{}{}{}.wav",
        sig_type, filename_freq, filename_duration, filename_ch
    );

    let override_msg = file_exists_errorcheck(&filename);

    FileInfo {
        name: filename,
        exists_msg: override_msg,
    }
}
