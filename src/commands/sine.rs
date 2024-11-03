use clap::Args;

#[derive(Args, Debug)]
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
