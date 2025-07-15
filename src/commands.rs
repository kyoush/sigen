use clap::{Parser, Subcommand};
use super::processing;

pub mod gen;
pub mod common;
pub mod taper;
pub mod wav;
pub mod modurate;
pub mod conv;

// default parameters
pub const AMP_MIN: f64 = 0.0;
pub const AMP_MAX: f64 = 1.0;
pub const AMP_DEF: f64 = 0.45;
pub const D_DEF: &str = "5"; // sec
pub const FREQ_DEF: i32 = 440; // Hz
pub const LOW_FREQ_TSP_DEF: i32 = 20; // Hz
pub const HIGH_FREQ_TSP_DEF: i32 = 16_000; // Hz
pub const PWM_FREQ_DEF: i32 = 200; // Hz
pub const PWM_DUTY_DEF: u32 = 10; // %
pub const FS_DEF: f64 = 44_100.0; // Hz
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
    /// generate a wav file
    Gen(gen::GenOptions),

    /// apply taper processing on existing wav file
    Taper(taper::TaperOptions),

    /// concatenates multiple WAV files into a single file
    Wav(wav::WavOptions),

    /// modurate a WAV file.
    Mod(modurate::ModOptions),

    /// convolution WAV files.
    Conv(conv::ConvOptions),
}
