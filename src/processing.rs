use std::error::Error;
use hound::WavSpec;
use crate::commands;
use crate::commands::gen::WaveFormCommands;
use crate::fileio;

pub mod gen;
mod cat;

const CH: u16 = 2; // stereo
const BITS_PER_SAMPLE: u16 = 16;

pub fn value_verify<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd + Copy,
{
    if min > max {
        panic!("unexpected error");
    }

    if value < min {
        min
    } else if max < value {
        max
    } else {
        value
    }
}

pub fn apply_taper_to_wav(options: &commands::taper::TaperOptions) -> Result<(), Box<dyn Error>> {
    let taper_spec = gen::get_taper_spec(Some(&options.taper_opt)).unwrap();
    let (mut samples, spec) = fileio::wavread::read_wav_file(options.input.as_str())?;
    let num_ch = samples.len();

    for i in 0..num_ch {
        rtaper::apply_taper_both(&mut samples[i], &taper_spec)?;
    }

    let fileinfo = crate::fileio::set_output_filename(options.output.clone(), options.input.as_str())?;
    fileio::wavwrite::write_wav_file(spec, fileinfo.name.as_str(), &samples, true, true)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}

pub fn signal_generator(args: commands::gen::GenOptions) -> Result<(), Box<dyn Error>> {
    let common_options = args.waveform.get_common_opt();
    let d = args.waveform.get_duration_in_sec()?;
    let taper_spec = args.waveform.get_taper_spec();
    let signal_spec = common_options.get_signal_spec(taper_spec, d);

    let (enable_l, enable_r, filename_ch) = match signal_spec.ch.as_str() {
        "L" => (true, false, "_l_only"),
        "R" => (false, true, "_r_only"),
        "LR" => (true, true, ""),
        _ => {
            return Err("unknown spec ch error".into());
        }
    };

    let (sig_type, startf, endf) = args.waveform.get_fileinfo(signal_spec.fs);

    let fileinfo = fileio::gen_file_name(
        &common_options.output_filename,
        sig_type,
        startf,
        endf,
        filename_ch,
        &args.waveform.get_duration_cmd(),
    )?;

    // generate signals
    let samples;
    match &args.waveform {
        WaveFormCommands::Sine(_) => {
            samples = gen::generate_sine_wave(&signal_spec, startf)?;
        }
        WaveFormCommands::Noise(noise_options) => {
            samples = gen::generate_noise(&signal_spec, &noise_options.noise_type)?;
        }
        WaveFormCommands::Tsp(tsp_options) => {
            samples = gen::generate_tsp_signal(&signal_spec, &tsp_options.tsp_type)?;
        }
        WaveFormCommands::Sweep(sweep_options) => {
            samples = gen::generate_sweep_signal(&signal_spec, &sweep_options.type_of_sweep, startf, endf)?;
        }
        WaveFormCommands::Pwm(pwm_options) => {
            let d_verified = value_verify(pwm_options.percent_of_duty, 0, 100);
            samples = gen::generate_pwm_signal(&signal_spec, startf, d_verified)?;
        }
        WaveFormCommands::Zeros(_) => {
            samples = gen::generate_zeros(&signal_spec)?;
        }
    }

    // write wav file
    let wav_spec = WavSpec {
        channels: CH,
        sample_rate: signal_spec.fs as u32,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    let samples_to_write = [samples.clone(), samples];

    fileio::wavwrite::write_wav_file(wav_spec, &fileinfo.name, &samples_to_write, enable_l, enable_r)?;

    println!("WAV file [{}] created successfully {}", fileinfo.name, fileinfo.exists_msg);

    Ok(())
}

pub fn cat_wav_files(options: &commands::wav::WavOptions) -> Result<(), Box<dyn Error>> {
    let (inputs, cat_cmd, output_filename) = options.parse_commands()?;
    let input_files: indexmap::IndexMap<String, String> = cat::parse_input_files(&inputs)?;
    let (spec, samples) = cat::parse_cat_commands(input_files, cat_cmd)?;

    let mut override_msg = String::new();
    if fileio::is_file_exist(output_filename.as_str()) {
        fileio::file_override_check(output_filename.as_str())?;
        override_msg = "(file override)".to_string();
    }

    fileio::wavwrite::write_wav_file(spec, output_filename.as_str(), &samples, true, true)?;

    println!("WAV file [{}] created successfully {}", output_filename, override_msg);

    Ok(())
}
