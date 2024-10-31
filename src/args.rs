use clap::{Args, Parser, Subcommand};

const AMP_DEF: f64 = 0.45;
const D_DEF_LONG: u32 = 30;        // sec
const D_DEF_SHORT: u32 = 5;        // sec
pub const FREQ_DEF: u32 = 440;     // Hz
const LOW_FREQ_TSP_DEF: i32 = 20;  // Hz
const HIGH_FREQ_TSP_DEF: i32 = 16_000; // Hz
const FS_DEF: u32 = 44_100;        // Hz
const LEN_TAPER_DEF: usize = 4096; //points

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

        /// duration of the signal in seconds.
        #[arg(
            short, long,
            default_value_t = D_DEF_LONG,
        )]
        duration: u32,

        #[command(flatten)]
        options: CommonOptions,
    },

    /// generate a wave file with a white noise
    White {
        /// duration of the signal in seconds.
        #[arg(
            short, long,
            default_value_t = D_DEF_LONG,
        )]
        duration: u32,

        #[command(flatten)]
        options: CommonOptions,
    },

    /// Generate a wave file with a TSP [Time Stretched Pulse] waveform
    Tsp {
        /// type of TSP signal waveform
        #[arg(
            short, long,
            default_value = "linear",
            value_parser = ["linear", "log"],
        )]
        tsp_type: String,

        /// duration of the signal in seconds.
        #[arg(
            short, long,
            default_value_t = D_DEF_SHORT,
        )]
        duration: u32,

        /// Starting frequency of the TSP signal in Hz
        #[arg(
            short, long,
            default_value_t = LOW_FREQ_TSP_DEF,
        )]
        startf: i32,

        /// Ending frequency of the TSP signal in Hz
        #[arg(
            short, long,
            default_value_t = HIGH_FREQ_TSP_DEF,
        )]
        endf: i32,

        #[command(flatten)]
        options: CommonOptions,
    },

    // Wav{} // To be Extended
}

#[derive(Args, Debug)]
pub struct CommonOptions {
    /// the maximum absolute value of the signal samplitude
    #[arg(
        short, long,
        default_value_t = AMP_DEF,
    )]
    pub amplitude: f64,

    /// Which channel generate
    #[arg(
        short, long,
        default_value = "LR",
        value_parser = ["L", "R", "LR"],
    )]
    pub channels: String,

    // Sample Rate of signal
    #[arg(
        short, long,
        default_value_t = FS_DEF,
    )]
    pub rate_of_sample: u32,

    // length of taper
    #[arg(
        short, long,
        default_value_t = LEN_TAPER_DEF,
    )]
    pub length_of_taper: usize,

    // type of taper
    #[arg(
        short, long,
        default_value = "linear",
        value_parser = ["linear", "hann", "cos", "blackman"]
    )]
    pub window_type: String,
}
