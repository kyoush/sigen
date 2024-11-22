use super::common;
use super::processing;
use clap::{Args, Subcommand};
use rtaper::TaperSpec;

const FREQ_DISABLE: i32 = -1;

#[derive(Args, Debug, Clone)]
pub struct GenOptions {
    #[command(subcommand)]
    pub waveform: WaveFormCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WaveFormCommands {
    /// generate a wav file with a sine wave
    Sine(SineOptions),

    /// generate a wav file with a noise
    Noise(NoiseOptions),

    /// generate a wav file with a TSP [Time Stretched Pulse] waveform
    Tsp(TspOptions),

    /// generate a wav file with a Swept-Sine
    Sweep(SweepOptions),

    /// generate a wav file with a PWM (pulse train)
    Pwm(PwmOptions),

    /// generate a wav file with zeros
    Zeros(ZerosOptions),
}

impl WaveFormCommands {
    pub fn get_common_opt(&self) -> &common::CommonOptions {
        match self {
            WaveFormCommands::Sine(opt) => &opt.options,
            WaveFormCommands::Noise(opt) => &opt.options,
            WaveFormCommands::Tsp(opt) => &opt.options,
            WaveFormCommands::Sweep(opt) => &opt.options,
            WaveFormCommands::Pwm(opt) => &opt.options,
            WaveFormCommands::Zeros(opt) => &opt.options,
        }
    }

    pub fn get_taper_spec(&self) -> Option<TaperSpec> {
        let opt = match self {
            WaveFormCommands::Sine(opt) => Some(&opt.taper_opt),
            WaveFormCommands::Noise(opt) => Some(&opt.taper_opt),
            WaveFormCommands::Tsp(opt) => Some(&opt.taper_opt),
            WaveFormCommands::Sweep(opt) => Some(&opt.taper_opt),
            WaveFormCommands::Pwm(opt) => Some(&opt.taper_opt),
            WaveFormCommands::Zeros(_) => None,
        };

        processing::gen::get_taper_spec(opt)
    }

    pub fn get_duration_cmd(&self) -> &String {
        match self {
            WaveFormCommands::Sine(opt) => &opt.duration,
            WaveFormCommands::Noise(opt) => &opt.duration,
            WaveFormCommands::Tsp(opt) => &opt.duration,
            WaveFormCommands::Sweep(opt) => &opt.duration,
            WaveFormCommands::Pwm(opt) => &opt.duration,
            WaveFormCommands::Zeros(opt) => &opt.duration,
        }
    }

    pub fn get_duration_in_sec(&self) -> Result<f64, Box<dyn std::error::Error>> {
        match self {
            WaveFormCommands::Sine(opt) => crate::processing::gen::parse_duration(&opt.duration),
            WaveFormCommands::Noise(opt) => crate::processing::gen::parse_duration(&opt.duration),
            WaveFormCommands::Tsp(opt) => crate::processing::gen::parse_duration(&opt.duration),
            WaveFormCommands::Sweep(opt) => crate::processing::gen::parse_duration(&opt.duration),
            WaveFormCommands::Pwm(opt) => crate::processing::gen::parse_duration(&opt.duration),
            WaveFormCommands::Zeros(opt) => crate::processing::gen::parse_duration(&opt.duration),
        }
    }

    pub fn get_fileinfo(&self, fs: i32) -> (String, i32, i32) {
        match self {
            WaveFormCommands::Sine(opt) => {
                let f = crate::processing::gen::parse_freq(&opt.frequency).unwrap();
                let f_verified = super::processing::value_verify(f, 0, fs / 2);
                ("sine".to_string(), f_verified, FREQ_DISABLE)
            }
            WaveFormCommands::Noise(opt) => {
                let noise_type = &opt.noise_type;
                let filename_type = format!("{}noise", noise_type);
                (filename_type, FREQ_DISABLE, FREQ_DISABLE)
            }
            WaveFormCommands::Tsp(_) => {
                ("tsp".to_string(), FREQ_DISABLE, FREQ_DISABLE)
            }
            WaveFormCommands::Sweep(opt) => {
                let filename_type: String = format!("{}_sweep", opt.type_of_sweep);
                let s = crate::processing::gen::parse_freq(&opt.startf).unwrap();
                let e = crate::processing::gen::parse_freq(&opt.endf).unwrap();
                (filename_type, s, e)
            }
            WaveFormCommands::Pwm(opt) => {
                let f_verified = super::processing::value_verify(opt.frequency, 0, fs / 2);
                ("pwm".to_string(), f_verified, FREQ_DISABLE)
            }
            WaveFormCommands::Zeros(_) => { ("zeros".to_string(), -1, -1) }
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct SineOptions {
    /// Frequency of the sine wave in Hz
    #[arg(
        short, long,
        default_value_t = super::FREQ_DEF.to_string(),
    )]
    pub frequency: String,

    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG.to_string(),
    )]
    pub duration: String,
}

#[derive(Args, Debug, Clone)]
pub struct NoiseOptions {
    // noise type
    #[arg(
        short, long,
        default_value = "white",
        value_parser = ["white", "pink"],
    )]
    pub noise_type: String,

    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG.to_string(),
    )]
    pub duration: String,
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

    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_SHORT.to_string(),
    )]
    pub duration: String,
}

#[derive(Args, Debug , Clone)]
pub struct SweepOptions {
    /// type of Swept-Sine waveform
    #[arg(
        short, long,
        default_value = "linear",
        value_parser = ["linear", "log"],
    )]
    pub type_of_sweep: String,

    /// Starting frequency of the Swept-Sine in Hz
    #[arg(
        short, long,
        default_value_t = super::LOW_FREQ_TSP_DEF.to_string(),
    )]
    pub startf: String,

    /// Ending frequency of the Swept-Sine in Hz
    #[arg(
        short, long,
        default_value_t = super::HIGH_FREQ_TSP_DEF.to_string(),
    )]
    pub endf : String,

    #[command(flatten)]
    pub options: common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: common::TaperSpecOptions,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG.to_string(),
    )]
    pub duration: String,
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

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG.to_string(),
    )]
    pub duration: String,
}

#[derive(Args, Debug, Clone)]
pub struct ZerosOptions {
    #[command(flatten)]
    pub options: common::CommonOptions,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_LONG.to_string(),
    )]
    pub duration: String,
}
