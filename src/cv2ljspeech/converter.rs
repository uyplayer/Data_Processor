use std::collections::HashMap;
use crate::cv2ljspeech::constants;


pub struct LJSpeech {
    abs_paths: Vec<String>,
    files: Vec<String>,
    output_file: HashMap<String, f32>,
}

#[derive(Debug)]
pub enum LJSpeechError {
    SumExceedsLimit(f32),
    OtherError(String),
}
impl Default for LJSpeech {
    fn default() -> Self {
        let mut default_output = HashMap::new();
        default_output.insert(String::from("training"), 0.8);
        default_output.insert(String::from("validation"), 0.2);
        LJSpeech {
            abs_paths: Vec::new(),
            files: Vec::new(),
            output_file: default_output,
        }
    }
}

impl std::fmt::Display for LJSpeechError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LJSpeechError::SumExceedsLimit(total_sum) => {
                write!(f, "Error: The sum of output_file values exceeds 1.0. Total: {}", total_sum)
            }
            LJSpeechError::OtherError(msg) => {
                write!(f, "Error: {}", msg)
            }
        }
    }
}
impl LJSpeech {
    pub fn new(abs_paths: Vec<String>, files: Vec<String>, output_file: HashMap<String, f32>) ->  Result<Self, LJSpeechError>{
        let total_sum = output_file.values().sum();
        if total_sum > 1.0 {
            return Err(LJSpeechError::SumExceedsLimit(total_sum));

        }
        let ljs = LJSpeech {
            abs_paths,
            files,
            output_file,
        };
        Ok(ljs)
    }

}

