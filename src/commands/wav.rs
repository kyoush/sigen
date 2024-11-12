use clap::Args;

#[derive(Args, Debug)]
pub struct WavOptions {
    #[arg(required = true)]
    inputs: Vec<String>,
}

const BREAK_PARSE_INPUTFILENAME: &str = "cat";
const BREAK_PARSE_CAT_COMMANDS: &str = "output";

impl WavOptions {
    pub fn parse_commands(&self) -> Result<(Vec<String>, Option<Vec<String>>, String), Box<dyn std::error::Error>>{
        let mut input_filenames: Vec<String> = Vec::new();
        let mut cat_commands: Vec<String> = Vec::new();
        let mut break_idx = 0;
        let mut break_idx_save = break_idx;

        for(i, input_str) in self.inputs.iter().enumerate() {
            if input_str == BREAK_PARSE_INPUTFILENAME {
                break_idx = i + 1;
                break;
            }
            input_filenames.push(input_str.to_string());
        }

        if break_idx == break_idx_save {
            return Err("cat command not given".into());
        }else {
            break_idx_save = break_idx;
        }

        for(i, input_str) in self.inputs.iter().enumerate().skip(break_idx) {
            if input_str == BREAK_PARSE_CAT_COMMANDS {
                break_idx = i + 1;
                break;
            }
            cat_commands.push(input_str.to_string());
        }

        if break_idx == break_idx_save {
            return Err("output command not given".into());
        }

        let cat_commands = if cat_commands.is_empty() {
            None
        }else {
            Some(cat_commands)
        };

        let output_filename = match self.inputs.get(break_idx) {
            Some(value) => value.to_string(),
            None => return Err("output filename is not given.".into()),
        };

        Ok((input_filenames, cat_commands, output_filename))
    }
}
