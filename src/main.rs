use clap::Parser;
use rtaper;
use hound::WavSpec;
use std::error::Error;

mod commands;
mod fileio;
mod processing;

const CH: u16 = 2; // stereo
const BITS_PER_SAMPLE: u16 = 16;
const AMP_MIN: f64 = 0.0;
const AMP_MAX: f64 = 1.0;

pub struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: u32,
    pub d: u32,
    pub taper_spec: rtaper::TaperSpec,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = commands::Cli::parse();

    let (common_options, taper_options, duration) = match &args.subcommand {
        commands::Commands::Sine(sine_options) => (
            Some(sine_options.options.clone()),
            &sine_options.taper_opt,
            sine_options.duration,
        ),
        commands::Commands::White(white_options) => (
            Some(white_options.options.clone()),
            &white_options.taper_opt,
            white_options.duration,
        ),
        commands::Commands::Tsp(tsp_options) => (
            Some(tsp_options.options.clone()),
            &tsp_options.taper_opt,
            tsp_options.duration,
        ),
        commands::Commands::Taper(taper_options) => (
            None,
            &taper_options.taper_opt,
            0,
        ),
    };

    let taper_type = match taper_options.window_type.as_str() {
        "linear" => { rtaper::WindowType::Linear }
        "hann" => { rtaper::WindowType::Hann }
        "cos" => { rtaper::WindowType::Cosine }
        "blackman" => { rtaper::WindowType::Blackman }
        _ => { rtaper::WindowType::Hann }
    };
    
    let taper_spec = rtaper::TaperSpec {
        taper_type: taper_type,
        taper_length: taper_options.length_of_taper,
    };

    if let commands::Commands::Taper(taper_options) = &args.subcommand {
        let fileinfo = processing::apply_taper_to_wav(&taper_options, &taper_spec)?;
        println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);
        return Ok(());
    }

    let (signal_spec, enable_l, enable_r, filename_ch) = match common_options {
        Some(ref common_options) =>  {
            let signal_spec = SignalSpec {
                amp: processing::value_verify(common_options.amplitude, AMP_MIN, AMP_MAX),
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
    match args.subcommand {
        commands::Commands::Sine(sine_options) => {
            let f_verified  = processing::value_verify(sine_options.frequency, 0, signal_spec.fs / 2);

            fileinfo = fileio::gen_file_name(
                &common_options.unwrap().output_filename,
                "sine",
                f_verified as i32,
                -1, filename_ch,
                signal_spec.d
            )?;
            samples = processing::generate_sine_wave(&signal_spec, f_verified);
        }
        commands::Commands::White(_) => {
            fileinfo = fileio::gen_file_name(
                &common_options.unwrap().output_filename,
                "white",
                -1,
                -1,
                filename_ch,
                signal_spec.d
            )?;
            samples = processing::generate_white_noise(&signal_spec);
        }
        commands::Commands::Tsp(tsp_options) => {
            let startf_verified = processing::value_verify(tsp_options.startf, 0, (signal_spec.fs / 2) as i32);
            let endf_verified = processing::value_verify(tsp_options.endf, 0, startf_verified);

            fileinfo = fileio::gen_file_name(
                &common_options.unwrap().output_filename,
                "tsp",
                startf_verified,
                endf_verified,
                filename_ch,
                signal_spec.d
            )?;
            samples = processing::generate_tsp_signal(&signal_spec, tsp_options.tsp_type, startf_verified, endf_verified);
        }
        _ => {
            return Err("unexpected command type".into());
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

    fileio::wavwrite::write_wav_file(wav_spec, &fileinfo.name, &samples_to_write, enable_l, enable_r)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}
