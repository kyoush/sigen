use std::error::Error;
use hound::WavSpec;
use super::fileio;
use indexmap::IndexMap;

pub fn parse_input_files(
    input_files_command: &Vec<String>
) -> Result<IndexMap<String, String>, Box<dyn Error>> {
    if input_files_command.is_empty() { return Err("input filename is not given".into()) }

    let mut input_files: IndexMap<String, String> = IndexMap::new();
    for (i, input_str) in input_files_command.iter().enumerate() {

        let key;
        let filename;

        if fileio::is_file_exist(input_str) {
            fileio::validate_wav_file(input_str)?;
            key = i.to_string();
            filename = input_str.clone();
        }else {
            if let Some(pos) = input_str.find('=') {
                key = input_str[..pos].to_string();
                filename = input_str[pos + 1..].to_string();

                fileio::validate_wav_file(&filename.as_str())?;
                fileio::validate_file_exist(&filename.as_str())?;
            }else {
                return Err(format!("File not found [{}]", input_str).into());
            }
        }
        input_files.insert(key, filename);
    }

    Ok(input_files)
}

fn append_stereo_signal(input: &Vec<Vec<f64>>, target: &mut Vec<Vec<f64>>) {
    for (i, sample) in input.iter().enumerate() {
        if let Some(s) = target.get_mut(i) {
            s.extend_from_slice(sample)
        } 
    }
}

fn concatenate_no_interval(
    input_files: IndexMap<String, String>,
    samples: &mut Vec<Vec<f64>>,
) -> Result<WavSpec, Box<dyn Error>> {
    let mut spec:  Option<WavSpec> = None;

    for(_, filename) in input_files {
        let (read_buf, wavspec) = fileio::wavread::read_wav_file(filename.as_str())?;
        spec = Some(wavspec);

        if samples.is_empty() {
            let num_ch = spec.unwrap().channels as usize;
            samples = vec![Vec::new(); num_ch];
        }else {
            append_stereo_signal(&read_buf, &mut samples);
        }
    }

    match spec {
        Some(_) => { return Ok((spec, samples)); }
        None => { return Err("WavSpec is not initialized".into()); }
    }
}

pub fn parse_cat_commands(input_files:IndexMap<String, String>, cat_cmd: Option<Vec<String>>) -> Result<(WavSpec, Vec<Vec<f64>>), Box<dyn Error>> {
    let mut samples: Vec<Vec<f64>> = Vec::new();
    let mut spec: Option<WavSpec> = None;

    match cat_cmd {
        Some(cmd) => {
            for hoge in cmd {
                println!("cat_cmd: {}", hoge);
            }
        }
        None => {
            (spec, samples) = concatenate_no_interval(input_files)?;
        }
    }

    match spec{
        Some(val) => { return Ok((val, samples)); }
        None => { return Err("WavSpec is not initialized".into()); }
    }
}
