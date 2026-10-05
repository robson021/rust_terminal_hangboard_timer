use crate::{sleep_seconds, sound};
use lazy_static::lazy_static;
use rodio::{source::Source, Decoder, OutputStream};
use sound::AudioNotification;
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::sync::{Arc, OnceLock};

#[inline(always)]
pub fn bell() {
    play_sound(AudioNotification::Bell);
}

#[inline(always)]
pub fn ding() {
    play_sound(AudioNotification::Ding);
}

#[inline(always)]
pub fn end_of_round() {
    play_sound(AudioNotification::RoundDone);
}

#[inline(always)]
pub fn finish() {
    play_sound(AudioNotification::Finish);
}

#[inline(always)]
pub fn get_ready() {
    play_sound(AudioNotification::GetReady);
}

lazy_static! {
    static ref AUDIO_THREAD_POOL: threadpool::ThreadPool = threadpool::ThreadPool::new(1);
    static ref AUDIO_FILES: HashMap<&'static str, OnceLock<Arc<[u8]>>> = [
        AudioNotification::Bell,
        AudioNotification::Ding,
        AudioNotification::Finish,
        AudioNotification::RoundDone,
        AudioNotification::GetReady,
    ]
    .into_iter()
    .map(|sound| (sound.to_file_path(), OnceLock::new()))
    .collect();
}

fn play_sound(sound: AudioNotification) {
    let file_path = sound.to_file_path();
    AUDIO_THREAD_POOL.execute(move || {
        let audio_data: Arc<[u8]> = AUDIO_FILES
            .get(file_path)
            .expect("Audio file is not registered")
            .get_or_init(|| {
                fs::read(file_path)
                    .unwrap_or_else(|_| panic!("Failed to open the file {file_path}"))
                    .into()
            })
            .clone();
        let decoder = Decoder::new(Cursor::new(audio_data))
            .unwrap_or_else(|_| panic!("Failed to decode the sound {sound:?}"));
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to open audio output stream");
        stream_handle
            .play_raw(decoder.convert_samples())
            .expect("Failed to play the sound");
        sleep_seconds(2);
    });
}
