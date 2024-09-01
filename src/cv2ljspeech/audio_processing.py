import os
import noisereduce as nr
import numpy as np
from pydub import AudioSegment
from pydub.silence import detect_leading_silence
from scipy.io import wavfile


def normalize_audio(data):
    max_val = np.max(np.abs(data))
    if max_val == 0:
        return data
    return data / max_val


def remove_silence(input_path: str):
    sound = AudioSegment.from_file(input_path, format="wav")
    start_trim = detect_leading_silence(sound)
    end_trim = detect_leading_silence(sound.reverse())
    duration = len(sound)
    trimmed_sound = sound[start_trim:duration - end_trim]
    trimmed_temp_path = "trimmed_temp.wav"
    trimmed_sound.export(trimmed_temp_path, format="wav")

    sample_rate, data = wavfile.read(trimmed_temp_path)
    reduced_noise = nr.reduce_noise(y=data, sr=sample_rate)
    normalized_noise = normalize_audio(reduced_noise)

    wavfile.write(input_path, sample_rate, np.int16(normalized_noise * 32767))
    print(f"Noise-reduced audio saved to {input_path}")

    os.remove(trimmed_temp_path)


def dir_fun(dirs: list[str]):
    for item in dirs:
        if not os.path.isdir(item):
            print(f"Skipping non-directory {item}")
            continue

        for audio_file in os.listdir(item):
            file_path = os.path.join(item, audio_file)
            if os.path.isfile(file_path):
                remove_silence(file_path)


if __name__ == "__main__":
    dir_list = [
        r"E:\MachineLearning\news_data\dev_clips",
        r"E:\MachineLearning\news_data\test_clips",
        r"E:\MachineLearning\news_data\train_clips",
        r"E:\MachineLearning\news_data\validated_clips",
    ]
    dir_fun(dir_list)
