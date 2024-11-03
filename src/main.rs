use std::process::exit;
use clap::Parser;
use rtaper;
use hound::WavSpec;

mod commands;
mod fileio;
mod processing;

const CH: u16 = 2; // stereo
const BITS_PER_SAMPLE: u16 = 16;

pub struct SignalSpec {
    pub amp: f64,
    pub ch: String,
    pub fs: u32,
    pub d: u32,
    pub taper_spec: rtaper::TaperSpec,
}

fn main() {
    let args = commands::Cli::parse();

    let (common_options, taper_options, duration) = match &args.subcommand {
        commands::Commands::Sine(sine_options) => (Some(sine_options.options.clone()), &sine_options.taper_opt, sine_options.duration),
        commands::Commands::White(white_options) => (Some(white_options.options.clone()), &white_options.taper_opt, white_options.duration),
        commands::Commands::Tsp(tsp_options) => (Some(tsp_options.options.clone()), &tsp_options.taper_opt, tsp_options.duration),
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

    let (signal_spec, enable_l, enable_r, filename_ch) = if let Some(common_options) = common_options {
        let signal_spec = SignalSpec {
            amp: common_options.amplitude,
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
                eprintln!("Error: unknown spec ch error");
                exit(1);
            }
        }
    }else {
        eprintln!("Error: common_options is not set");
        exit(1);
    };

    // generate signals
    let samples;
    let fileinfo;
    match args.subcommand {
        commands::Commands::Sine(sine_options) => {
            fileinfo = fileio::gen_file_name("sine", sine_options.frequency as i32, -1, filename_ch, signal_spec.d);
            samples = processing::generate_sine_wave(&signal_spec, sine_options.frequency);
        }
        commands::Commands::White(_) => {
            fileinfo = fileio::gen_file_name("white", -1, -1, filename_ch, signal_spec.d);
            samples = processing::generate_white_noise(&signal_spec);
        }
        commands::Commands::Tsp(tsp_options) => {
            fileinfo = fileio::gen_file_name("tsp", tsp_options.startf, tsp_options.endf, filename_ch, signal_spec.d);
            samples = processing::generate_tsp_signal(&signal_spec, tsp_options.tsp_type, tsp_options.startf, tsp_options.endf);
        }
        _ => {
            eprintln!("Error: unexpected command type");
            exit(1);
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

    if let Err(e) = fileio::wavwrite::write_wav_file(wav_spec, &fileinfo.name, &samples_to_write, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file \"{}\" created successfully{}", fileinfo.name, fileinfo.exists_msg);
    exit(0);
}
