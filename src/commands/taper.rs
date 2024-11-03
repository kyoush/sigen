use clap::Args;

#[derive(Args, Debug)]
pub struct TaperOptions {
    /// input filename
    pub input: String,

    ///  Output filename.
    /// If specified without an argument, input file will be overridden.
    #[arg(short, long)]
    pub output: Option<Option<String>>,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpecOptions,
}
