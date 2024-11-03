use clap::Args;

#[derive(Args, Debug)]
pub struct TaperOptions {
    /// input filename
    pub input: String,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpecOptions,
}
