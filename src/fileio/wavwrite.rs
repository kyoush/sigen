use std::error::Error;
use hound::{WavSpec, WavWriter};

use super::is_wav_file;

pub fn write_wav_file(
    spec: WavSpec,
    filename: &str,
    samples: &[Vec<f64>],
    enable_l: bool,
    enable_r: bool,
) -> Result<(), Box<dyn Error>> {
    if !is_wav_file(filename) {
        return Err(format!("The filename must have a .wav extension. [{}]", filename).into());
    }

    let mut writer = WavWriter::create(filename, spec)?;

    let total_samples = samples[0].len();
    let num_ch = samples.len();

    for i in 0..total_samples {
        for j in 0 .. num_ch {
            let enable = if num_ch % 2 == 0 {
                enable_l as i16
            }else {
                enable_r as i16
            };

            let sample_value: i16 =
            (samples[j][i] * i16::MAX as f64).clamp(i16::MIN as f64, i16::MAX as f64) as i16;
            writer.write_sample(sample_value * enable)?;
        }
    }

    writer.flush()?;
    Ok(())
}
