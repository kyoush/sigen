use hound::{WavSpec, WavWriter};

pub fn write_wav_file(
    fs: u32,
    filename: &str,
    samples: &[f64],
    channels: u32,
    enable_l: bool,
    enable_r: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: channels as u16,
        sample_rate: fs,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(filename, spec)?;

    let total_samples = samples.len();

    for i in 0..total_samples {
        let sample_value: i16 =
            (samples[i] * i16::MAX as f64).clamp(i16::MIN as f64, i16::MAX as f64) as i16;
        writer.write_sample(sample_value * (enable_l as i16))?;
        writer.write_sample(sample_value * (enable_r as i16))?;
    }

    writer.flush()?;
    Ok(())
}
