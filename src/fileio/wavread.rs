use hound::{WavReader, WavSpec, Error};

pub fn read_wav_file(filename: &str) -> Result<(Vec<Vec<f64>>, WavSpec), Error> {
    let mut reader = WavReader::open(filename)?;
    let spec = reader.spec();
    let num_channels = spec.channels as usize;
    let mut samples = vec![Vec::new(); num_channels];

    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample?;
        let channel = i % num_channels;
        samples[channel].push(sample as f64 / i16::MAX as f64);
    }

    Ok((samples, spec))
}
