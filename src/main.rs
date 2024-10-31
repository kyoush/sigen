use std::process::exit;
use clap::Parser;
use rtaper;

mod args;
mod funcs;
mod generate;

const CH: u32 = 2; // stereo

fn main() {
    let args = args::Cli::parse();

    let (common_options, duration) = match &args.subcommand {
        args::Commands::Sine { options, duration, ..} => (options, *duration),
        args::Commands::White { options, duration, ..} => (options, *duration),
        args::Commands::Tsp { options, duration, ..} => (options, *duration),
    };

    let taper_type = match common_options.window_type.as_str() {
        "linear" => { rtaper::WindowType::Linear }
        "hann" => { rtaper::WindowType::Hann }
        "cos" => { rtaper::WindowType::Cosine }
        "blackman" => { rtaper::WindowType::Blackman }
        _ => { rtaper::WindowType::Hann }
    };
    
    let taper_spec = rtaper::TaperSpec {
        taper_type: taper_type,
        taper_length: common_options.length_of_taper,
    };

    let spec = crate::generate::SignalSpec {
        amp: common_options.amplitude,
        ch: common_options.channels.clone(),
        fs: common_options.rate_of_sample,
        d: duration,
        taper_spec: taper_spec,
    };

    let enable_l;
    let enable_r;
    let filename_ch;
    match spec.ch.as_str() {
        "L" => {
            enable_l = true;
            enable_r = false;
            filename_ch = "_l_only";
        },
        "R" => {
            enable_l = false;
            enable_r = true;
            filename_ch = "_r_only";
        },
        "LR" => {
            enable_l = true;
            enable_r = true;
            filename_ch = "";
        },
        _ => {
            eprintln!("Error: unknown type of channels");
            exit(1);
        }
    }

    // generate signals
    let samples;
    let fileinfo;
    match args.subcommand {
        args::Commands::Sine { frequency, .. } => {
            fileinfo = funcs::gen_file_name("sine", frequency as i32, -1, filename_ch, spec.d);
            samples = generate::generate_sine_wave(&spec, frequency);
        }
        args::Commands::White { .. } => {
            fileinfo = funcs::gen_file_name("white", -1, -1, filename_ch, spec.d);
            samples = generate::generate_white_noise(&spec);
        }
        args::Commands::Tsp { tsp_type, startf, endf, ..} => {
            fileinfo = funcs::gen_file_name("tsp", startf, endf, filename_ch, spec.d);
            samples = generate::generate_tsp_signal(&spec, tsp_type, startf, endf);
        }
    }

    // write wav file
    if let Err(e) = funcs::write_wav_file(&spec, &fileinfo.name, &samples, CH, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file \"{}\" created successfully{}", fileinfo.name, fileinfo.exists_msg);
    exit(0);
}
