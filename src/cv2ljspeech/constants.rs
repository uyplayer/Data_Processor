

/// The sample rate for audio files, set to 22050 Hz for LJSpeech dataset.
pub const SAMPLE_RATE: u32 = 22050;

/// The bit depth for audio files, set to 16-bit PCM for LJSpeech dataset.
pub const BIT_DEPTH: u16 = 16;

/// The number of audio channels, set to 1 for mono audio files in LJSpeech dataset.
pub const CHANNELS: u16 = 1;

/// The audio format used for the files, set to "WAV" as per LJSpeech dataset requirements.
pub const AUDIO_FORMAT: &str = "WAV";

/// The encoding used for the audio files, set to "PCM" as required by LJSpeech dataset.
pub const ENCODING: &str = "PCM";
