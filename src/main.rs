use std::process::exit;
use std::path::Path;
use std::io::{self, Write};
use clap::Parser;

mod funcs;
mod args;

const CH: u32 = 2; // stereo
const N: usize = 4096; // samples of taper

fn main() {
    let args = args::Args::parse();
    let mut enable_l = true;
    let mut enable_r = true;
    let mut filename_ch = "";
    let mut override_msg = "";

    if args.channels == "L" {
        enable_l = true;
        enable_r = false;
        filename_ch = "_l_only";
    }else if args.channels == "R" {
        enable_l = false;
        enable_r = true;
        filename_ch = "_r_only";
    }
    let filename = funcs::gen_file_name(&args.type_sig, args.frequency, filename_ch, args.duration);

    // -------------- ERROR CHECK -------------- //
    // freq cannot be specified when whitenoise
    if args.type_sig == "white" {
        if args.frequency != args::FREQ_DEF {
            eprintln!("Error: Frequency cannot be specified when type_sig is 'white'");
            exit(1);
        }
    }

    // file exist check
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
                eprintln!("file already exists, canceled.");
                exit(1);
            }
            _ => {
                override_msg = " (file override)";
            }
        }
    }
    // ------------ ERROR CHECK [end] ------------ //

    // Generate samples
    let mut samples = match args.type_sig.as_str() {
        "sine" => funcs::generate_sine_wave(args.amplitude, args.duration, args.frequency),
        "white" => funcs::generate_white_noise(args.amplitude, args.duration),
        _ => {
            eprintln!("Error: Invalid type_sig");
            exit(1);
        }
    };
    funcs::apply_linear_taper(&mut samples, N);

    // write wav file
    if let Err(e) = funcs::write_wav_file(&filename, &samples, CH, enable_l, enable_r) {
        eprintln!("Error writing WAV file: {}", e);
        exit(1);
    }

    println!("WAV file \"{}\" created successfully{}", filename, override_msg);
    exit(0);
}
