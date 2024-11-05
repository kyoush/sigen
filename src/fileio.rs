use std::path::Path;
use std::io::{self, Write};

pub mod wavread;
pub mod wavwrite;

pub const RIFF_HEADER_SIZE: usize = 44; // bytes
const FILESIZE_WARN_LEVEL: usize = 1_000_000_000; // 1GB

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

fn file_override_check(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    print!("Do you want to overwrite [{}]? [y/N] ", filename);
    io::stdout().flush().unwrap();
    if io::stdin().read_line(&mut input).is_err() {
        return Err("Failed to reading input".into());
    }

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            Ok(())
        }
        _ => {
            return Err("The operation was canceled by user.".into());
        }
    }
}

fn output_filesize_check(filesize: usize) -> Result<(), Box<dyn std::error::Error>> {
    if filesize > FILESIZE_WARN_LEVEL {
        let mut input = String::new();
        print!(
            "The output file size will approximately {:.1} GB. Proceed? [y/N] ",
            filesize as f64 / 1024.0 / 1024.0 / 1024.0
        );
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                return Ok(());
            }
            _ => {
                return Err("The operation was canceled by user".into());
            }
        }
    }else {
        Ok(())
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

fn seconds_format(sec: i32) -> String{
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
    d: i32) -> Result<FileInfo, Box<dyn std::error::Error>> {
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

pub fn set_output_filename(output_filename: Option<Option<String>>, input_filename: &str) -> Result<FileInfo, Box<dyn std::error::Error>> {
    let mut fileinfo = FileInfo{
        name: String::new(),
        exists_msg: String::new(),
    };
    match output_filename {
        Some(Some(ref name)) if !name.is_empty() => {
            if is_file_exist(name) {
                file_override_check(name)?;
                fileinfo.exists_msg = "(file override)".to_string();
            }
            fileinfo.name = name.clone();
        }
        Some(Some(_)) => {
            return Err("The output filename is empty!".into());
        }
        Some(None) => {
            if is_file_exist(input_filename) {
                file_override_check(input_filename)?;
                fileinfo.exists_msg = " (file override)".to_string();
            }
            fileinfo.name = input_filename.to_string();
        }
        None => {
            let default_name = format!("{}_tapered.wav", extract_stem(input_filename));
            if is_file_exist(&default_name) {
                file_override_check(&default_name)?;
                fileinfo.exists_msg = " (file override)".to_string();
            }
            fileinfo.name = default_name;
        }
    }

    if is_wav_file(fileinfo.name.as_str()) {
        Ok(fileinfo)
    }else {
        return Err(format!("the filename must have a .wav extension. [{}]", fileinfo.name).into());

    }
}
