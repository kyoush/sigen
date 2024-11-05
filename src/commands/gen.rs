use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct GenOptions {
    #[command(subcommand)]
    pub waveform: WaveFormCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WaveFormCommands {
    /// generate a wav file with a sine wave
    Sine(SineOptions),

    /// generate a wave file with a white noise
    White(WhiteOptions),

    /// generate a wave file with a TSP [Time Stretched Pulse] waveform
    Tsp (TspOptions),

    // Pwm(PwmOptions), // To be Extended
}

#[derive(Args, Debug, Clone)]
pub struct SineOptions {
    /// Frequency of the sine wave in Hz
    #[arg(
        short, long,
        default_value_t = super::FREQ_DEF,
    )]
    pub frequency: u32,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG,
    )]
    pub duration: u32,

    #[command(flatten)]
    pub options: super::common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpecOptions,
}

#[derive(Args, Debug, Clone)]
pub struct WhiteOptions {
    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG,
    )]
    pub duration: u32,

    #[command(flatten)]
    pub options: super::common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpecOptions,
}

#[derive(Args, Debug, Clone)]
pub struct TspOptions {
    /// type of TSP signal waveform
    #[arg(
        short, long,
        default_value = "linear",
        value_parser = ["linear", "log"],
    )]
    pub tsp_type: String,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_SHORT,
    )]
    pub duration: u32,

    /// Starting frequency of the TSP signal in Hz
    #[arg(
        short, long,
        default_value_t = super::LOW_FREQ_TSP_DEF,
    )]
    pub startf: i32,

    /// Ending frequency of the TSP signal in Hz
    #[arg(
        short, long,
        default_value_t = super::HIGH_FREQ_TSP_DEF,
    )]
    pub endf: i32,

    #[command(flatten)]
    pub options: super::common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpecOptions,
}
