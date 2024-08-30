use std::fs::File;
use std::{fs, io, path};
use std::path::Path;
use csv::ReaderBuilder;

pub struct LJSpeech {
    pub abs_paths: Vec<String>,
    pub dev: bool,
    pub train: bool,
    pub test: bool,
    pub validated: bool,
    pub output_location: String,
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
    pub fn new(
        abs_paths: Vec<String>,
        output_location: Option<String>,
        dev: Option<bool>,
        train: Option<bool>,
        test: Option<bool>,
        validated: Option<bool>
    ) -> Result<Self, LJSpeechError> {
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
                return Err(LJSpeechError::FileNotFoundError(format!("Data path '{}' does not exist", path)));
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

    pub fn make_metadata(&self) -> io::Result<()> {
        let index = 1;
        for file_dir in &self.abs_paths {
            let parent_path = Path::new(&file_dir);
            if self.dev {
                let dev = parent_path.join("dev.tsv");
                self.create_directory(parent_path.join("dev_clips").as_path())?;
            }
            if self.train {
                let train = parent_path.join("train.tsv");
                self.create_directory(parent_path.join("train_clips").as_path())?;
            }
            if self.test {
                let test = parent_path.join("test.tsv");
                self.create_directory(parent_path.join("test_clips").as_path())?;
            }
            if self.validated {
                let validated = parent_path.join("validated.tsv");
                self.create_directory(parent_path.join("validated_clips").as_path())?;
            }
        }
        Ok(())
    }

    fn create_directory(&self, dir_path: &Path) -> io::Result<()> {
        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
            println!("Directory created successfully at {:?}", dir_path);
        } else {
            println!("Directory already exists at {:?}", dir_path);
        }
        Ok(())
    }

    pub fn read_tsv(
        &self,
        dir_path: &Path,
        file_path: &Path,
        which_file: String
    ) -> Result<Vec<String>, io::Error> {
        let mut index = 1;
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(file);
        let mut tsv_result: Vec<String> = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let path = &record[1];
            let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
            let sentence = &record[2];
            let audio_location = dir_path.join("clips").join(path);
            let new_name = which_file.clone() + "_" + &index.to_string();
            let new_audio_location = Path::new(&self.output_location)
                .join(which_file.clone() + "_clips")
                .join(new_name.clone() + "." + extension);
            if audio_location.exists() {
                fs::copy(&audio_location, &new_audio_location)?;
            } else {
                eprintln!("Source file does not exist: {:?}", audio_location);
            }
            let line = new_name.clone() + "|" + sentence;
            tsv_result.push(line);
            index += 1;
        }
        Ok(tsv_result)
    }
}
