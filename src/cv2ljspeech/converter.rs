use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use crate::cv2ljspeech::constants;

pub struct LJSpeech {
    pub abs_paths: Vec<String>,
    pub dev: bool,
    pub train: bool,
    pub test: bool,
    pub validated: bool,
    output_location: String,
}

#[derive(Debug)]
pub enum LJSpeechError {
    DirError(String),
    FileNotFoundError(String),
    OtherError(String),
}

impl Default for LJSpeech {
    fn default() -> Self {
        LJSpeech {
            abs_paths: Vec::new(),
            dev: true,
            train: true,
            test: true,
            validated: true,
            output_location: ".".to_string(),
        }
    }
}

impl LJSpeech {
    pub fn new(abs_paths: Vec<String>,output_location: Option<String>,dev: Option<bool>,train: Option<bool>,test: Option<bool>,validated: Option<bool>) -> Result<Self, LJSpeechError> {
        let dev = dev.unwrap_or(true);
        let train = train.unwrap_or(true);
        let test = test.unwrap_or(false);
        let validated = validated.unwrap_or(false);
        let output_location = output_location.unwrap_or_else(|| ".".to_string());
        if !(dev || train || test || validated) {
            return Err(LJSpeechError::OtherError(
                "At least one of dev, train, test, or validated must be true".to_string(),
            ));
        }
        let output_path = Path::new(&output_location);
        if !output_path.exists() || !output_path.is_dir() {
            return Err(LJSpeechError::DirError(
                format!("Output location '{}' does not exist or is not a directory", output_location),
            ));
        }
        for path in &abs_paths {
            let abs_path = Path::new(path);
            if !abs_path.exists() {
                return Err(LJSpeechError::FileNotFoundError(format!("Data path '{}' does not exist", path),
                ));
            }
        }

        let ljs = LJSpeech {
            abs_paths,
            dev,
            train,
            test,
            validated,
            output_location,
        };
        Ok(ljs)
    }
    pub fn print_info(&self) {
        println!("LJSpeech Information:");
        println!("  Paths: {:?}", self.abs_paths);
        println!("  Output Location: {}", self.output_location);
        println!("  Dev Mode: {}", self.dev);
        println!("  Train Mode: {}", self.train);
        println!("  Test Mode: {}", self.test);
        println!("  Validated: {}", self.validated);
    }
}
