use hound::{WavSpec, WavWriter};

use super::is_wav_file;

pub fn write_wav_file(
    spec: WavSpec,
    filename: &str,
    samples: &[Vec<f64>],
    enable_l: bool,
    enable_r: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !is_wav_file(filename) {
        return Err(format!("The filename must have a .wav extension. [{}]", filename).into());
    }

    let mut writer = WavWriter::create(filename, spec)?;

    let samples_per_ch = samples[0].len();
    let num_ch = samples.len();

    let output_filesize = (samples_per_ch * num_ch) * std::mem::size_of::<i16>() + super::RIFF_HEADER_SIZE;
    super::output_filesize_check(output_filesize)?;

    for i in 0..samples_per_ch {
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
