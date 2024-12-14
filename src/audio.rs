use crate::sleep;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

const BELL_SOUND: &str = "assets/sounds/bell.wav";
const DING_SOUND: &str = "assets/sounds/ding.wav";
const FINISH: &str = "assets/sounds/finish.wav";

#[inline(always)]
pub fn play_bell() {
    play_sound(BELL_SOUND);
}

#[inline(always)]
pub fn play_ding() {
    play_sound(DING_SOUND);
}

#[inline(always)]
pub fn play_finish() {
    play_sound(FINISH);
}

fn play_sound(filename: &str) {
    // todo: cache the file, instead of loading every time
    let file = BufReader::new(File::open(filename).expect("File not found"));
    let source = Decoder::new(file).expect("Failed to decode the file");
    let samples = source.convert_samples();
    // todo: run in a common thread pool
    thread::spawn(|| {
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to open audio output stream");
        stream_handle
            .play_raw(samples)
            .expect("Failed to play the sound");
        sleep(2);
    });
}
