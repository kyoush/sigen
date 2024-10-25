use clap::{Parser};

const AMP_DEF: f64 = 0.45;
const D_DEF: u32 = 30; // sec
pub const FREQ_DEF: u32 = 440; // Hz

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(
        short, long,
        default_value_t = AMP_DEF,
        help = "Amplitude of the signal",
    )]
    pub amplitude: f64,

    #[arg(
        short, long,
        default_value_t = D_DEF,
        help = "Duration of the signal in seconds",
    )]
    pub duration: u32,

    #[arg(
        short, long,
        default_value = "sine",
        value_parser = ["sine", "white"],
        help = "Type of the waveform",
    )]
    pub type_sig: String,
    

    #[arg(
        short, long,
        default_value = "LR",
        value_parser = ["L", "R", "LR"],
        help = "Which channel generate"
    )]
    pub channels: String,
    
    #[arg(
        short, long,
        default_value_t = FREQ_DEF,
        help = "Frequency of the sine wave in Hz",
    )]
    pub frequency: u32,
}
