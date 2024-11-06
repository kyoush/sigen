use clap::{Args, Subcommand};
use super::common;
use rtaper::TaperSpec;
use super::processing;

#[derive(Args, Debug, Clone)]
pub struct GenOptions {
    #[command(subcommand)]
    pub waveform: WaveFormCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WaveFormCommands {
    /// generate a wav file with a sine wave
    Sine(SineOptions),

    /// generate a wav file with a white noise
    White(WhiteOptions),

    /// generate a wav file with a TSP [Time Stretched Pulse] waveform
    Tsp(TspOptions),

    /// generate a wav file with a PWM (pulse train)
    Pwm(PwmOptions),
}

impl WaveFormCommands {
    pub fn get_common_opt(&self) -> &common::CommonOptions {
        match self {
            WaveFormCommands::Sine(opt) => &opt.options,
            WaveFormCommands::White(opt) => &opt.options,
            WaveFormCommands::Tsp(opt) => &opt.options,
            WaveFormCommands::Pwm(opt) => &opt.options,
        }
    }

    pub fn get_taper_spec(&self) -> TaperSpec {
        let opt = match self {
            WaveFormCommands::Sine(opt) => &opt.taper_opt,
            WaveFormCommands::White(opt) => &opt.taper_opt,
            WaveFormCommands::Tsp(opt) => &opt.taper_opt,
            WaveFormCommands::Pwm(opt) => &opt.taper_opt,
        };

        processing::get_taper_spec(opt)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SineOptions {
    /// Frequency of the sine wave in Hz
    #[arg(
        short, long,
        default_value_t = super::FREQ_DEF,
    )]
    pub frequency: i32,

    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,
}

#[derive(Args, Debug, Clone)]
pub struct WhiteOptions {
    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,
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
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,
}

#[derive(Args, Debug, Clone)]
pub struct PwmOptions {
    /// Frequency of PWM in Hz
    #[arg(
        short, long,
        default_value_t = super::PWM_FREQ_DEF,
    )]
    pub frequency: i32,

    /// Duty cycle of PWM in %
    #[arg(
        short, long,
        default_value_t = super::PWM_DUTY_DEF,
    )]
    pub percent_of_duty: u32,

    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,
}
