use std::process::exit;
use clap::Parser;
use funcs::gen_file_name;

mod args;
mod funcs;

const CH: u32 = 2; // stereo
const N: usize = 4096; // samples of taper

fn main() {
    let args = args::Cli::parse();
    let mut enable_l = true;
    let mut enable_r = true;
    let mut filename_ch = "";

    let common_options = match &args.subcommand {
        args::Commands::Sine { options, ..} => options,
        args::Commands::White { options, ..} => options,
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
    let mut samples;
    let fileinfo;
    match args.subcommand {
        args::Commands::Sine { frequency, options } => {
            fileinfo = gen_file_name("sine", frequency as i32, -1, filename_ch, options.duration);
            samples = funcs::generate_sine_wave(options.amplitude, options.duration, frequency);
        }
        args::Commands::White { options } => {
            fileinfo = gen_file_name("white", -1, -1, filename_ch, options.duration);
            samples = funcs::generate_white_noise(options.amplitude, options.duration);
        }
    }
    funcs::apply_linear_taper(&mut samples, N);

    // write wav file
    if let Err(e) = funcs::write_wav_file(&fileinfo.name, &samples, CH, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file \"{}\" created successfully{}", fileinfo.name, fileinfo.exists_msg);
    exit(0);
}
