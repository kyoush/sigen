use clap::Args;
use rtaper::TaperSpec; 
use crate::processing::gen::SignalSpec;

#[derive(Args, Clone, Debug)]
pub struct CommonOptions {
    /// the maximum absolute value of the signal samplitude
    #[arg(
        short, long,
        default_value_t = super::AMP_DEF,
    )]
    pub amplitude: f64,

    /// Which channel generate
    #[arg(
        short, long,
        default_value = "LR",
        value_parser = ["L", "R", "LR"],
    )]
    pub channels: String,

    /// Sample Rate of signal
    #[arg(
        short, long,
        default_value_t = super::FS_DEF,
    )]
    pub rate_of_sample: i32,

    /// Output Filename
    #[arg(
        short, long,
    )]
    pub output_filename: Option<String>,
}

impl CommonOptions {
    pub fn get_signal_spec(&self, taper_spec: Option<TaperSpec>, d: f64) -> SignalSpec {
        SignalSpec {
            amp: super::processing::value_verify(self.amplitude, super::AMP_MIN, super::AMP_MAX),
            ch: self.channels.clone(),
            fs: self.rate_of_sample,
            d: d,
            taper_spec: taper_spec,
        }
    }
}

#[derive(Args, Clone, Debug)]
pub struct TaperSpecOptions {
    /// length of taper
    /// set this to zero to disable tapering
    #[arg(
        short, long,
        default_value_t = super::LEN_TAPER_DEF,
    )]
    pub length_of_taper: usize,

    /// type of taper
    #[arg(
        short, long,
        default_value = "linear",
        value_parser = ["linear", "hann", "cos", "blackman"]
    )]
    pub window_type: String,
}
