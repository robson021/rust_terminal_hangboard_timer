use crate::sound;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use sound::AudioNotification;
use std::fs;
use std::io::Cursor;
use std::sync::{Arc, OnceLock};

static AUDIO_THREAD_POOL: OnceLock<threadpool::ThreadPool> = OnceLock::new();
static AUDIO_FILES: [OnceLock<Arc<[u8]>>; 5] = [const { OnceLock::new() }; 5];

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

fn play_sound(sound: AudioNotification) {
    let file_path = sound.to_file_path();
    let file_index = match sound {
        AudioNotification::Bell => 0,
        AudioNotification::Ding => 1,
        AudioNotification::Finish => 2,
        AudioNotification::RoundDone => 3,
        AudioNotification::GetReady => 4,
    };
    AUDIO_THREAD_POOL
        .get_or_init(|| threadpool::ThreadPool::new(1))
        .execute(move || {
            let audio_data = AUDIO_FILES[file_index]
                .get_or_init(|| {
                    fs::read(file_path)
                        .unwrap_or_else(|_| panic!("Failed to open the file {file_path}"))
                        .into()
                })
                .clone();
            let decoder = Decoder::try_from(Cursor::new(audio_data))
                .unwrap_or_else(|_| panic!("Failed to decode the sound {sound:?}"));
            let mut stream =
                DeviceSinkBuilder::open_default_sink().expect("Failed to open audio output stream");
            stream.log_on_drop(false);
            let player = Player::connect_new(stream.mixer());
            player.append(decoder);
            player.sleep_until_end();
        });
}
