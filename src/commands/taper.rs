use clap::Args;

#[derive(Args, Debug)]
pub struct TaperOptions {
    /// the input filename
    pub filename: String,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpecOptions,
}
