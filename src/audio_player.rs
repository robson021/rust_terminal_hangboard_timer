use crate::{sleep_seconds, sound};
use lazy_static::lazy_static;
use rodio::{source::Source, Decoder, OutputStream};
use sound::AudioNotification;
use std::fs::File;
use std::io::BufReader;

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

#[inline(always)]
fn open_file(filename: &str) -> File {
    File::open(filename).unwrap_or_else(|_| panic!("Failed to open the file {filename}"))
}

lazy_static! {
    static ref AUDIO_THREAD_POOL: threadpool::ThreadPool = threadpool::ThreadPool::new(1);
}

fn play_sound(sound: AudioNotification) {
    let file_path = sound.to_file_path();
    AUDIO_THREAD_POOL.execute(move || {
        let file = open_file(file_path);
        let buf_reader = BufReader::new(file);
        let decoder = Decoder::new(buf_reader)
            .unwrap_or_else(|_| panic!("Failed to decode the sound {sound:?}"));
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to open audio output stream");
        stream_handle
            .play_raw(decoder.convert_samples())
            .expect("Failed to play the sound");
        sleep_seconds(2);
    });
}
