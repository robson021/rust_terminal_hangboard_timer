pub enum AudioNotification {
    Bell,
    Ding,
    Finish,
    RoundDone,
}
impl AudioNotification {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioNotification::Bell => "assets/sounds/bell.wav",
            AudioNotification::Ding => "assets/sounds/ding.wav",
            AudioNotification::Finish => "assets/sounds/finish.wav",
            AudioNotification::RoundDone => "assets/sounds/round_done.wav",
        }
    }
}
