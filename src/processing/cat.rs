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
        let (read_buf, tmp_spec) = fileio::read_wav_file(filename.as_str())?;

        if spec.is_none() {
            spec = Some(tmp_spec);
        }

        if samples.is_empty() {
            samples.extend(read_buf);
        }else {
            append_stereo_signal(&read_buf, samples);
        }
    }

    spec.ok_or_else(|| "spec is not set".into())
}

fn append_zeros(
    duration: &f64,
    fs: &u32,
    samples: &mut Vec<Vec<f64>>,
) {
    let points = (*duration * *fs as f64) as usize;
    for col in samples.iter_mut() {
        col.extend(std::iter::repeat(0.0).take(points));
    }
}

fn is_specify_key(cat_commands: &Vec<String>) -> bool{
    cat_commands.iter().any(|cmd| {
        let first = cmd.chars().next().unwrap().to_string();
        first.parse::<i32>().is_err()
    })
}

fn do_cat_commands(
    cmd: Vec<String>,
    samples: &mut Vec<Vec<f64>>,
    filemap: IndexMap<String, String>,
) -> Result<WavSpec, Box<dyn Error>>{
    let mut spec: Option<WavSpec> = None;
    let flag = is_specify_key(&cmd);

    let mut i: usize = 0;
    for (_, cat_command) in cmd.iter().enumerate() {
        let first = cat_command.chars().next().unwrap().to_string();
        let (filename, duration) = match first.parse::<i32>() {
            Ok(_) => { // specify duration
                let d = crate::processing::gen::parse_duration(cat_command)?;

                if i < filemap.len() && !flag {
                    let (_, filename) = filemap.get_index(i).unwrap();
                    (Some(filename.clone()), Some(d))
                }else {
                    (None, Some(d))
                }
            }
            Err(_) => { // specify key
                let f = filemap.get(cat_command).ok_or_else(|| format!("key: [{}] is not found", cat_command))?;
                (Some(f.clone()), None)
            }
        };

        if filename.is_some() {
            let (read_buf, tmp_spec) = fileio::read_wav_file(filename.unwrap().as_str())?;

            if spec.is_none() {
                spec = Some(tmp_spec);
            }

            if samples.is_empty() {
                *samples = read_buf;
            }else {
                append_stereo_signal(&read_buf, samples);
            }
        }

        if duration.is_some() {
            if spec.is_none() {
                let (_, tmp_filename) = filemap.get_index(0).unwrap();
                let (_, tmp_spec) = fileio::read_wav_file(tmp_filename)?;
                spec = Some(tmp_spec);
            }

            if samples.is_empty() {
                let points = (duration.unwrap() * spec.unwrap().sample_rate as f64) as usize;
                *samples = vec![vec![0.0; points]; spec.unwrap().channels as usize];
            }else {
                append_zeros(&duration.unwrap(), &spec.unwrap().sample_rate, samples);
            }
        }

        i += 1;
    }

    while i < filemap.len() && !flag {
        let (_, f) = filemap.get_index(i).unwrap();
        let (read_buf, tmp_spec) = fileio::read_wav_file(f)?;

        if spec.is_none() {
            spec = Some(tmp_spec);
        }

        append_stereo_signal(&read_buf, samples);

        i += 1;
    }

    spec.ok_or_else(|| "spec is not set".into())
}

pub fn parse_cat_commands(
    input_files:IndexMap<String, String>,
    cat_cmd: Option<Vec<String>>,
) -> Result<(WavSpec, Vec<Vec<f64>>), Box<dyn Error>> {
    let mut samples: Vec<Vec<f64>> = Vec::new();

    let spec = match cat_cmd {
        Some(cat_commands_vec) => {
            do_cat_commands(cat_commands_vec, &mut samples, input_files)?
        }
        None => {
            concatenate_no_interval(input_files, &mut samples)?
        }
    };

    Ok((spec, samples))
}
