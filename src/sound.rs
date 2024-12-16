#[derive(Eq, PartialEq, Hash, Debug)]
pub enum AudioNotification {
    Bell,
    Ding,
    Finish,
    RoundDone,
}
impl AudioNotification {
    pub fn to_file_path(&self) -> &'static str {
        match self {
            AudioNotification::Bell => "assets/sound/bell.wav",
            AudioNotification::Ding => "assets/sound/ding.wav",
            AudioNotification::Finish => "assets/sound/finish.wav",
            AudioNotification::RoundDone => "assets/sound/round_done.wav",
        }
    }
}
