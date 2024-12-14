use crate::sleep;
use rodio::source::SamplesConverter;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub const BELL_SOUND: &str = "assets/sounds/bell.wav";
pub const DING_SOUND: &str = "assets/sounds/ding.wav";

pub struct Audio {
    samples: SamplesConverter<Decoder<BufReader<File>>, f32>,
}

pub trait PlayableAudio {
    fn play_sound(self);
}

impl PlayableAudio for Audio {
    fn play_sound(self) {
        // todo: play in common thread pool
        thread::spawn(move || {
            let (_stream, stream_handle) =
                OutputStream::try_default().expect("Failed to open audio output stream");
            stream_handle
                .play_raw(self.samples)
                .expect("Failed to play the sound");
            sleep(1);
        });
    }
}

pub fn get_audio(filename: &str) -> Audio {
    let file = BufReader::new(File::open(filename).expect("File not found"));
    let source = Decoder::new(file).expect("Failed to decode the file");
    let samples = source.convert_samples();
    Audio { samples }
}
