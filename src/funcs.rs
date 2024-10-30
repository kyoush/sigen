use hound::{WavSpec, WavWriter};
use std::path::Path;
use std::io::{self, Write};
use std::process::exit;
use crate::generate::SignalSpec;

pub struct FileInfo {
    pub name: String,
    pub exists_msg: String,
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
