use std::error::Error;
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

pub fn validate_wav_file(filename: &str) -> Result<(), Box<dyn Error>> {
    if !is_wav_file(filename) {
        return Err(format!("The filename must have a .wav extension. [{}]", filename).into());
    }

    Ok(())
}

pub fn is_file_exist(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn validate_file_exist(filename: &str) -> Result<(), Box<dyn Error>> {
    if !is_file_exist(filename) {
        return Err(format!("File [{}] not found.", filename).into());
    }

    Ok(())
}

pub fn file_override_check(filename: &str) -> Result<(), Box<dyn Error>> {
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

fn output_filesize_check(filesize: usize) -> Result<(), Box<dyn Error>> {
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

fn duration_format(d_cmd: &str) -> String {
    match d_cmd.parse::<i32>() {
        Ok(val) => {
            if val >= 60 {
                if val % 60 == 0 {
                    format!("_{}min", val / 60)
                } else {	
                    format!("_{}min{}s", val / 60, val % 60)
                }
            } else {
                format!("_{}s", val)
            }
        }
        Err(_) => {
            format!("_{}", d_cmd)
        }
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
    d_cmd: &str) -> Result<FileInfo, Box<dyn Error>> {
    let filename = if let Some(name) = output_filename {
        name.clone()
    }else {
        let filename_start_freq = freq_format(start_freq, "");
        let filename_end_freq = freq_format(end_freq, "to_");
        let filename_duration = duration_format(d_cmd);

        format!(
            "{}{}{}{}{}.wav",
            sig_type, filename_start_freq, filename_end_freq, filename_duration, filename_ch
        )
    };

    validate_wav_file(&filename)?;

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
    let mut fileinfo = FileInfo{
        name: String::new(),
        exists_msg: String::new(),
    };

    let mut enable_file_exists_check = true;

    let filename = match output_filename {
        Some(Some(ref name)) if !name.is_empty() => name.clone(), // use user specify name
        Some(Some(_)) => return Err("The output filename is empty!".into()),
        Some(None) => {
            enable_file_exists_check = false;
            input_filename.to_string() // input file will override
        }
        None => format!("{}_tapered.wav", extract_stem(input_filename)), // use default name
    };

    validate_wav_file(filename.as_str())?;
    fileinfo.name = filename.clone();

    if enable_file_exists_check {
        if is_file_exist(&filename) {
            file_override_check(&filename)?;
            fileinfo.exists_msg = "(file override)".to_string();
        }
    }else {
        file_override_check(&filename)?;
        fileinfo.exists_msg = "(file override)".to_string();
    }

    Ok(fileinfo)
}
