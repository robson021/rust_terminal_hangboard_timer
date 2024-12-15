use crate::{sleep, sound};
use rodio::{source::Source, Decoder, OutputStream};
use sound::AudioNotification;
use std::fs::File;
use std::io::BufReader;
use std::thread;

#[inline(always)]
pub fn bell() {
    play_sound(AudioNotification::Bell);
}

#[inline(always)]
pub fn ding() {
    play_sound(AudioNotification::Ding);
}

#[inline(always)]
pub fn finish() {
    play_sound(AudioNotification::Finish);
}

#[inline(always)]
fn get_audio_samples(filename: &str) -> Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(filename).expect("File not found"));
    Decoder::new(file).unwrap_or_else(|_| panic!("Failed to decode the file {}", filename))
}

// todo: cache the file, instead of loading every time & run in a common thread pool
fn play_sound(sound: AudioNotification) {
    let filename = sound.as_str();
    let source = get_audio_samples(filename);
    thread::spawn(|| {
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to open audio output stream");
        stream_handle
            .play_raw(source.convert_samples())
            .expect("Failed to play the sound");
        sleep(2);
    });
}
