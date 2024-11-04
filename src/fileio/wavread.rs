use std::error::Error;
use hound::{WavReader, WavSpec};

use super::is_wav_file;

pub fn read_wav_file(filename: &str) -> Result<(Vec<Vec<f64>>, WavSpec), Box<dyn Error>> {
    if !is_wav_file(filename) {
        return Err(format!("the filename must have a .wav extension. [{}]", filename).into());
    }

    let mut reader =  WavReader::open(filename)?;
    let spec = reader.spec().clone();
    let num_channels = spec.channels as usize;
    let mut samples = vec![Vec::new(); num_channels];

    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample?;
        let channel = i % num_channels;
        samples[channel].push(sample as f64 / i16::MAX as f64);
    }

    Ok((samples, spec))
}
