use clap::{Parser, Subcommand};

pub mod common;
pub mod sine;
pub mod white;
pub mod tsp;
pub mod taper;

pub const AMP_DEF: f64 = 0.45;
pub const D_DEF_LONG: u32 = 30;        // sec
pub const D_DEF_SHORT: u32 = 5;        // sec
pub const FREQ_DEF: u32 = 440;     // Hz
pub const LOW_FREQ_TSP_DEF: i32 = 20;  // Hz
pub const HIGH_FREQ_TSP_DEF: i32 = 16_000; // Hz
pub const FS_DEF: u32 = 44_100;        // Hz
pub const LEN_TAPER_DEF: usize = 4096; //points

/// A tool for generating WAV files of various signal types.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// generate a wav file with a sine wave
    Sine(sine::SineOptions),

    /// generate a wave file with a white noise
    White(white::WhiteOptions),

    /// generate a wave file with a TSP [Time Stretched Pulse] waveform
    Tsp (tsp::TspOptions),

    /// apply taper processing on existing wav file
    Taper(taper::TaperOptions),

    // Wav(wav::WavOptions), // To be Extended
}
