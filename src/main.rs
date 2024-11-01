use std::process::exit;
use clap::Parser;
use rtaper;

mod commands;
mod fileio;
mod processing;

const CH: u32 = 2; // stereo

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


    let (spec, enable_l, enable_r, filename_ch) = if let Some(common_options) = common_options {
        let spec = SignalSpec {
            amp: common_options.amplitude,
            ch: common_options.channels.clone(),
            fs: common_options.rate_of_sample,
            d: duration,
            taper_spec: taper_spec,
        };

        match spec.ch.as_str() {
            "L" => (spec, true, false, "l_only"),
            "R" => (spec, false, true, "_r_only"),
            "LR" => (spec, true, true, ""),
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
            fileinfo = fileio::gen_file_name("sine", sine_options.frequency as i32, -1, filename_ch, spec.d);
            samples = processing::generate_sine_wave(&spec, sine_options.frequency);
        }
        commands::Commands::White(_) => {
            fileinfo = fileio::gen_file_name("white", -1, -1, filename_ch, spec.d);
            samples = processing::generate_white_noise(&spec);
        }
        commands::Commands::Tsp(tsp_options) => {
            fileinfo = fileio::gen_file_name("tsp", tsp_options.startf, tsp_options.endf, filename_ch, spec.d);
            samples = processing::generate_tsp_signal(&spec, tsp_options.tsp_type, tsp_options.startf, tsp_options.endf);
        }
    }

    // write wav file
    if let Err(e) = fileio::wavwrite::write_wav_file(spec.fs, &fileinfo.name, &samples, CH, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file \"{}\" created successfully{}", fileinfo.name, fileinfo.exists_msg);
    exit(0);
}
