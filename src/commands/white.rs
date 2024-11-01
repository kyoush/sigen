use clap::Args;

#[derive(Args, Debug)]
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
    pub taper_opt: super::common::TaperSpec,
}
