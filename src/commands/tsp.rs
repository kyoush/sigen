use clap::Args;

#[derive(Args, Debug)]
pub struct TspOptions {
    /// type of TSP signal waveform
    #[arg(
        short, long,
        default_value = "linear",
        value_parser = ["linear", "log"],
    )]
    pub tsp_type: String,

    /// duration of the signal in seconds.
    #[arg(
        short, long,
        default_value_t = super::D_DEF_SHORT,
    )]
    pub duration: u32,

    /// Starting frequency of the TSP signal in Hz
    #[arg(
        short, long,
        default_value_t = super::LOW_FREQ_TSP_DEF,
    )]
    pub startf: i32,

    /// Ending frequency of the TSP signal in Hz
    #[arg(
        short, long,
        default_value_t = super::HIGH_FREQ_TSP_DEF,
    )]
    pub endf: i32,

    #[command(flatten)]
    pub options: super::common::CommonOptions,

    #[command(flatten)]
    pub taper_opt: super::common::TaperSpec,
}
