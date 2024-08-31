use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use crate::cv2ljspeech::converter::LJSpeechError;
use crate::cv2ljspeech::constants;

pub struct Audio {
    pub abs_paths: Vec<String>,
}

enum AudioFormat {
    Wav,
    Mp3,
    Unknown,
}

impl Audio {
    pub fn new(abs_paths: Vec<String>) -> Result<Self, LJSpeechError> {
        let audio = Audio { abs_paths };
        Ok(audio)
    }

    pub fn read_dir(&self) {
        for file_dir in &self.abs_paths {
            println!("Start Handling {:?}", file_dir);
            if let Err(e) = self.read_audio_dir(file_dir) {
                eprintln!("Error reading files in {:?}: {}", file_dir, e);
            }
            println!("End Handling {:?}", file_dir);
        }
    }

    pub fn read_audio_dir(&self, file_dir: &str) -> Result<(), io::Error> {
        let file_dir = Path::new(file_dir);
        if file_dir.is_dir() {
            for entry in fs::read_dir(file_dir)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    self.read_audio_file(&entry.path());
                } else if file_type.is_dir() {
                    println!("Directory: {:?}", entry.path());
                }
            }
        }
        Ok(())
    }

    pub fn read_audio_file(&self, file_path: &Path) {
        if let Some(extension) = file_path.extension() {
            match extension.to_str() {
                Some("wav") => self.handle_audio_format(AudioFormat::Wav, file_path),
                Some("mp3") => self.handle_audio_format(AudioFormat::Mp3, file_path),
                _ => self.handle_audio_format(AudioFormat::Unknown, file_path),
            }
        } else {
            self.handle_audio_format(AudioFormat::Unknown, file_path);
        }
    }

    fn handle_audio_format(&self, format: AudioFormat, file_path: &Path) {
        match format {
            AudioFormat::Wav | AudioFormat::Mp3 => {
                let output_path = file_path.with_extension("wav");
                if let Err(e) = self.convert_mp3_to_wav(file_path, &output_path) {
                    eprintln!("Error converting file {:?}: {}", file_path, e);
                } else {
                    if let AudioFormat::Mp3 = format {
                        if let Err(e) = fs::remove_file(file_path) {
                            eprintln!("Error deleting MP3 file: {}", e);
                        } else {
                            println!("Deleted original MP3 file: {:?}", file_path);
                        }
                    }
                }
            }
            AudioFormat::Unknown => println!("Unknown format file: {:?}", file_path),
        }
    }

    fn convert_mp3_to_wav(&self, input_path: &Path, output_path: &Path) -> io::Result<()> {
        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-ar")
            .arg(constants::SAMPLE_RATE.to_string())
            .arg("-ac")
            .arg(constants::CHANNELS.to_string())
            .arg("-sample_fmt")
            .arg("s16")
            .arg(output_path)
            .status()?;

        if status.success() {
            println!("Successfully converted {:?} to {:?}", input_path, output_path);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to convert audio file"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cv2ljspeech::audio::Audio;

    #[test]
    fn audio_test() {
        let paths: Vec<String> = vec![
            r"C:\Users\uyplayer\Downloads\validated_clips".to_string(),
            r"C:\Users\uyplayer\Downloads\test_clips".to_string(),
            r"C:\Users\uyplayer\Downloads\train_clips".to_string(),
            r"C:\Users\uyplayer\Downloads\dev_clips".to_string(),
        ];
        let audio = Audio { abs_paths: paths };

        audio.read_dir();
    }
}
