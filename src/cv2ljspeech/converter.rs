use std::sync::{Arc, Mutex};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use csv::ReaderBuilder;

pub struct LJSpeech {
    pub abs_paths: Vec<String>,
    pub dev: bool,
    pub train: bool,
    pub test: bool,
    pub validated: bool,
    pub output_location: String,
    index_dev: Arc<Mutex<i32>>,
    index_train: Arc<Mutex<i32>>,
    index_test: Arc<Mutex<i32>>,
    index_validated: Arc<Mutex<i32>>,
}

#[derive(Debug)]
pub enum LJSpeechError {
    DirError(String),
    FileNotFoundError(String),
    FormatError(String),
    OtherError(String),
}

impl LJSpeech {
    pub fn new(
        abs_paths: Vec<String>,
        output_location: Option<String>,
        dev: Option<bool>,
        train: Option<bool>,
        test: Option<bool>,
        validated: Option<bool>,
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
            index_dev: Arc::new(Mutex::new(1)),
            index_train: Arc::new(Mutex::new(1)),
            index_test: Arc::new(Mutex::new(1)),
            index_validated: Arc::new(Mutex::new(1)),
        };
        Ok(ljs)
    }

    pub fn make_metadata(&self) -> io::Result<()> {
        let out_dir = Path::new(&self.output_location);
        for file_dir in &self.abs_paths {
            let parent_path = Path::new(&file_dir);
            let file_dir = Path::new(file_dir);
            println!("Start Handling {:?}",&*file_dir);
            if self.dev {
                let dev = parent_path.join("dev.tsv");
                let dev_clips_dir = out_dir.join("dev_clips");
                self.create_directory(dev_clips_dir.as_path())?;
                let index_dev = Arc::clone(&self.index_dev);
                let dev_res = self.read_tsv(file_dir, &*dev, "dev".to_string(), index_dev)?;
                let dev_txt = out_dir.join("dev.txt");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(dev_txt)?;

                for line in dev_res {
                    writeln!(file, "{}", line)?;
                }
            }

            if self.train {
                let train = parent_path.join("train.tsv");
                let train_clips_dir = out_dir.join("train_clips");
                self.create_directory(train_clips_dir.as_path())?;
                let index_train = Arc::clone(&self.index_train);
                let train_res = self.read_tsv(file_dir, &*train, "train".to_string(), index_train)?;
                let train_txt = out_dir.join("train.txt");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(train_txt)?;

                for line in train_res {
                    writeln!(file, "{}", line)?;
                }
            }

            if self.test {
                let test = parent_path.join("test.tsv");
                let test_clips_dir = out_dir.join("test_clips");
                self.create_directory(test_clips_dir.as_path())?;
                let index_test = Arc::clone(&self.index_test);
                let test_res = self.read_tsv(file_dir, &*test, "test".to_string(), index_test)?;
                let test_txt = out_dir.join("test.txt");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(test_txt)?;

                for line in test_res {
                    writeln!(file, "{}", line)?;
                }
            }

            if self.validated {
                let validated = parent_path.join("validated.tsv");
                let validated_clips_dir = out_dir.join("validated_clips");
                self.create_directory(validated_clips_dir.as_path())?;
                let index_validated = Arc::clone(&self.index_validated);
                let validated_res = self.read_tsv(file_dir, &*validated, "validated".to_string(), index_validated)?;
                let validated_txt = out_dir.join("validated.txt");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(validated_txt)?;

                for line in validated_res {
                    writeln!(file, "{}", line)?;
                }
            }
            println!("End Handling {:?}",&*file_dir)
        }

        Ok(())
    }

    pub fn read_tsv(
        &self,
        dir_path: &Path,
        file_path: &Path,
        which_file: String,
        index: Arc<Mutex<i32>>,
    ) -> Result<Vec<String>, io::Error> {
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(file);
        let mut tsv_result: Vec<String> = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let path = &record[1];
            let path = Path::new(path);
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
            let mut sentence = record[2].to_string();
            if sentence.starts_with('.') {
                sentence = sentence.trim_start_matches('.').to_string();
            }
            if !sentence.ends_with('.') {
                sentence.push('.');
            }

            let audio_location = dir_path.join("clips").join(path);
            let new_name = which_file.clone() + "_" + &*index.lock().unwrap().to_string();
            let new_audio_location = Path::new(&self.output_location)
                .join(which_file.clone() + "_clips")
                .join(new_name.clone() + "." + extension);
            if audio_location.exists() {
                fs::copy(&audio_location, &new_audio_location)?;
            } else {
                eprintln!("Source file does not exist: {:?}", audio_location);
            }

            let line = new_name.clone() + "|" + &sentence;
            tsv_result.push(line);
            let mut index_lock = index.lock().unwrap();
            *index_lock += 1;
        }
        Ok(tsv_result)
    }

    fn create_directory(&self, path: &Path) -> io::Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn ljs_test() {
        let paths: Vec<String> = vec![
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-11.0-2022-09-21-ug\cv-corpus-11.0-2022-09-21\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-12.0-2022-12-07-ug\cv-corpus-12.0-2022-12-07\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-13.0-2023-03-09-ug\cv-corpus-13.0-2023-03-09\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-14.0-2023-06-23-ug\cv-corpus-14.0-2023-06-23\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-15.0-2023-09-08-ug\cv-corpus-15.0-2023-09-08\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-16.1-2023-12-06-ug\cv-corpus-16.1-2023-12-06\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-17.0-2024-03-15-ug\cv-corpus-17.0-2024-03-15\ug".to_string(),
            r"E:\MachineLearning\datasets\mozilacommoncoive\original_data\cv-corpus-18.0-2024-06-14-ug\cv-corpus-18.0-2024-06-14\ug".to_string()
        ];

        let out_dir = r"E:\MachineLearning\news_data";
        let ljs = LJSpeech {
            abs_paths: paths,
            dev: true,
            train: true,
            test: true,
            validated: true,
            output_location: out_dir.to_string(),
            index_dev: Arc::new(Mutex::new(0)),
            index_train: Arc::new(Mutex::new(0)),
            index_test: Arc::new(Mutex::new(0)),
            index_validated: Arc::new(Mutex::new(0)),
        };
        ljs.print_info();
        ljs.make_metadata().unwrap();
    }
}
