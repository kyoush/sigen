use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct ModOptions {
    #[arg(required = true)]
    inputs: Vec<String>,
}
