use std::path::Path;
use std::io::{self, Write};
use std::error::Error;

pub mod wavread;
pub mod wavwrite;

pub struct FileInfo {
    pub name: String,
    pub exists_msg: String,
}

fn is_wav_file(filename: &str) -> bool {
    Path::new(filename)
        .extension()
        .map(|e| e == "wav")
        .unwrap_or(false)
}

fn is_file_exist(filename: &str) -> bool {
    let exists = Path::new(filename).exists();
    if exists{
        println!("The file [{}] already exists.", filename);
    }
    exists
}

fn file_override_check(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    print!("The file [{}] will be overridden. Are you sure? [y/N] ", filename);
    io::stdout().flush().unwrap();
    if io::stdin().read_line(&mut input).is_err() {
        return Err("failed to reading input".into());
    }

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            Ok(())
        }
        _=> {
            return Err("The operation was canceled by user.".into());
        }
    }
}

fn freq_format(freq: i32, prefix: &str) -> String {
    if freq < 0 {
        return String::new();
    }
    
    if freq < 1000 {
        format!("_{}{}hz", prefix, freq)
    } else {
        format!("_{}{}khz", prefix, freq / 1000)
    }
}

fn seconds_format(sec: u32) -> String{
    if sec >= 60 {
        if sec % 60 == 0 {
            format!("_{}min", sec / 60)
        } else {
            format!("_{}min{}s", sec / 60, sec % 60)
        }
    } else {
        format!("_{}s", sec)
    }
}

fn extract_stem(input_filename: &str) -> String {
    Path::new(input_filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("").to_string()
}

pub fn gen_file_name(
    output_filename: &Option<String>,
    sig_type: &str,
    start_freq: i32,
    end_freq: i32,
    filename_ch: &str,
    d: u32) -> Result<FileInfo, Box<dyn Error>> {
    let filename = if let Some(name) = output_filename {
        name.clone()
    }else {
        let filename_start_freq = freq_format(start_freq, "");
        let filename_end_freq = freq_format(end_freq, "to_");
        let filename_duration = seconds_format(d);

        format!(
            "{}{}{}{}{}.wav",
            sig_type, filename_start_freq, filename_end_freq, filename_duration, filename_ch
        )
    };

    if !is_wav_file(filename.as_str()) {
        return Err(format!("the filename must have a .wav extension. [{}]", filename).into());
    }

    let mut override_msg = String::new();
    if is_file_exist(&filename) {
        file_override_check(&filename)?;
        override_msg = "(file override)".to_string();
    }

    Ok(FileInfo {
        name: filename,
        exists_msg: override_msg,
    })
}

pub fn set_output_filename(output_filename: Option<Option<String>>, input_filename: &str) -> Result<FileInfo, Box<dyn Error>> {
    let mut override_msg = "";
    match output_filename {
        Some(Some(ref name)) if !name.is_empty() => { // @todo .wavの拡張子がついているかチェックを追加する
            if is_file_exist(name) {
                file_override_check(name)?;
                override_msg = "(file override)";
            }
            Ok(FileInfo {
                    name: name.clone(),
                    exists_msg: override_msg.to_string(),
            })
        }
        Some(Some(_)) => {
            return Err("The output filename is empty!".into());
        }
        Some(None) => {
            if is_file_exist(input_filename) {
                file_override_check(input_filename)?;
                override_msg = " (file override)";
            }
            Ok(FileInfo {
                name: input_filename.to_string(),
                exists_msg: override_msg.to_string(),
            })
        }
        None => {
            let default_name = format!("{}_tapered.wav", extract_stem(input_filename));
            if is_file_exist(&default_name) {
                file_override_check(&default_name)?;
                override_msg = " (file override)";
            }
            Ok(FileInfo {
                name: default_name,
                exists_msg: override_msg.to_string(),
            })
        }
    }
}
