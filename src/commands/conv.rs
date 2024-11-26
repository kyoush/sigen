use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct ConvOptions {
    #[arg(required = true)]
    input: String,
}
