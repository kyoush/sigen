use clap::{Args, Parser, Subcommand};

const AMP_DEF: f64 = 0.45;
const D_DEF: u32 = 30; // sec
pub const FREQ_DEF: u32 = 440; // Hz

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
    Sine {
        /// Frequency of the sine wave in Hz
        #[arg(
            short, long,
            default_value_t = FREQ_DEF,
        )]
        frequency: u32,

        #[command(flatten)]
        options: CommonOptions,
    },

    /// generate a wave file with a white noise
    White {
        #[command(flatten)]
        options: CommonOptions,
    }
}

#[derive(Args, Debug)]
pub struct CommonOptions {
    /// the maximum absolute value of the signal samplitude
    #[arg(
        short, long,
        default_value_t = AMP_DEF,
    )]
    pub amplitude: f64,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = D_DEF,
    )]
    pub duration: u32,

    /// Which channel generate
    #[arg(
        short, long,
        default_value = "LR",
        value_parser = ["L", "R", "LR"],
    )]
    pub channels: String,
}
