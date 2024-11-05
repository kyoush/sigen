use clap::Parser;

mod commands;
mod fileio;
mod processing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = commands::Cli::parse();

    match args.subcommand {
        commands::Commands::Gen(opt) => {
            processing::signal_generator(&opt)?;
        }
        commands::Commands::Taper(opt) => {
            processing::apply_taper_to_wav(&opt)?;
        }
    }

    Ok(())
}
