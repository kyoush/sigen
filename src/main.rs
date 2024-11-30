use clap::Parser;

mod commands;
mod fileio;
mod processing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match commands::Cli::parse().subcommand {
        commands::Commands::Gen(opt) => processing::signal_generator(&opt),
        commands::Commands::Taper(opt) => processing::apply_taper_to_wav(&opt),
        commands::Commands::Wav(opt) => processing::cat_wav_files(&opt),
        commands::Commands::Mod(opt) => processing::wav_modurate(&opt),
    }
}
