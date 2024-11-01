use clap::Args;

#[derive(Args, Clone, Debug)]
pub struct CommonOptions {
    /// the maximum absolute value of the signal samplitude
    #[arg(
        short, long,
        default_value_t = super::AMP_DEF,
    )]
    pub amplitude: f64,

    /// Which channel generate
    #[arg(
        short, long,
        default_value = "LR",
        value_parser = ["L", "R", "LR"],
    )]
    pub channels: String,

    // Sample Rate of signal
    #[arg(
        short, long,
        default_value_t = super::FS_DEF,
    )]
    pub rate_of_sample: u32,
}

#[derive(Args, Debug)]
pub struct TaperSpec {
    /// length of taper
    /// ゼロを渡したら、テーパー処理が無効化されます
    #[arg(
        short, long,
        default_value_t = super::LEN_TAPER_DEF,
    )]
    pub length_of_taper: usize,

    // type of taper
    #[arg(
        short, long,
        default_value = "linear",
        value_parser = ["linear", "hann", "cos", "blackman"]
    )]
    pub window_type: String,
}
