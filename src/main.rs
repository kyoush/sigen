use std::process::exit;
use clap::Parser;
use funcs::gen_file_name;

mod args;
mod funcs;

const CH: u32 = 2; // stereo

fn main() {
    let args = args::Cli::parse();
    let mut enable_l = true;
    let mut enable_r = true;
    let mut filename_ch = "";

    let common_options = match &args.subcommand {
        args::Commands::Sine { options, ..} => options,
        args::Commands::White { options, ..} => options,
        args::Commands::Tsp { options, ..} => options,
    };

    if common_options.channels == "L" {
        enable_l = true;
        enable_r = false;
        filename_ch = "_l_only";
    }else if common_options.channels == "R" {
        enable_l = false;
        enable_r = true;
        filename_ch = "_r_only";
    }

    // generate signals
    let samples;
    let fileinfo;
    match args.subcommand {
        args::Commands::Sine { frequency, duration, options } => {
            fileinfo = gen_file_name("sine", frequency as i32, -1, filename_ch, duration);
            samples = funcs::generate_sine_wave(options.amplitude, duration, frequency);
        }
        args::Commands::White { duration, options } => {
            fileinfo = gen_file_name("white", -1, -1, filename_ch, duration);
            samples = funcs::generate_white_noise(options.amplitude, duration);
        }
        args::Commands::Tsp { tsp_type, duration, startf, endf, options } => {
            fileinfo = gen_file_name("tsp", startf, endf, filename_ch, duration);
            samples = funcs::generate_tsp_signal(options.amplitude, duration, tsp_type, startf, endf);
        }
    }

    // write wav file
    if let Err(e) = funcs::write_wav_file(&fileinfo.name, &samples, CH, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file \"{}\" created successfully{}", fileinfo.name, fileinfo.exists_msg);
    exit(0);
}
