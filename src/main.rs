use std::process::exit;
use clap::Parser;
use funcs::{gen_file_name, SignalSpec};

mod args;
mod funcs;

const CH: u32 = 2; // stereo

fn main() {
    let args = args::Cli::parse();
    let mut enable_l = true;
    let mut enable_r = true;
    let mut filename_ch = "";

    let (common_options, duration) = match &args.subcommand {
        args::Commands::Sine { options, duration, ..} => (options, *duration),
        args::Commands::White { options, duration, ..} => (options, *duration),
        args::Commands::Tsp { options, duration, ..} => (options, *duration),
    };

    let spec = SignalSpec {
        amp: common_options.amplitude,
        ch: common_options.channels.clone(),
        fs: common_options.rate_of_sample,
        d: duration,
    };

    if spec.ch == "L" {
        enable_l = true;
        enable_r = false;
        filename_ch = "_l_only";
    }else if spec.ch == "R" {
        enable_l = false;
        enable_r = true;
        filename_ch = "_r_only";
    };

    // generate signals
    let samples;
    let fileinfo;
    match args.subcommand {
        args::Commands::Sine { frequency, .. } => {
            fileinfo = gen_file_name("sine", frequency as i32, -1, filename_ch, spec.d);
            samples = funcs::generate_sine_wave(&spec, frequency);
        }
        args::Commands::White { .. } => {
            fileinfo = gen_file_name("white", -1, -1, filename_ch, spec.d);
            samples = funcs::generate_white_noise(&spec);
        }
        args::Commands::Tsp { tsp_type, startf, endf, ..} => {
            fileinfo = gen_file_name("tsp", startf, endf, filename_ch, spec.d);
            samples = funcs::generate_tsp_signal(&spec, tsp_type, startf, endf);
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
