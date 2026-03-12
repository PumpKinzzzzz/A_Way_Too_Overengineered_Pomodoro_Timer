#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sequence {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct Settings {
    pub work_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub auto_start_breaks: bool,
    pub sequence_list: Vec<Sequence>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            work_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            auto_start_breaks: true,
            sequence_list: vec![Sequence::Work, Sequence::ShortBreak, Sequence::Work, Sequence::LongBreak],
        }
    }

    pub fn update_work_duration(&mut self, duration: u64) {
        self.work_duration = duration;
    }

    pub fn update_short_break_duration(&mut self, duration: u64) {
        self.short_break_duration = duration;
    }

    pub fn update_long_break_duration(&mut self, duration: u64) {
        self.long_break_duration = duration;
    }

    pub fn toggle_auto_start_breaks(&mut self) {
        self.auto_start_breaks = !self.auto_start_breaks;
    }

    pub fn update_sequence_list(&mut self, sequence_list: Vec<Sequence>) {
        self.sequence_list = sequence_list;
    }
}

#[test]
fn test_settings() {
    let mut settings = Settings::new();
    assert_eq!(settings.work_duration, 25);
    settings.update_work_duration(30);
    assert_eq!(settings.work_duration, 30);

    assert_eq!(settings.short_break_duration, 5);
    settings.update_short_break_duration(10);
    assert_eq!(settings.short_break_duration, 10);

    assert_eq!(settings.long_break_duration, 15);
    settings.update_long_break_duration(20);
    assert_eq!(settings.long_break_duration, 20);

    assert_eq!(settings.auto_start_breaks, true);
    settings.toggle_auto_start_breaks();
    assert_eq!(settings.auto_start_breaks, false);

    let new_sequence = vec![Sequence::Work, Sequence::Work, Sequence::ShortBreak];
    settings.update_sequence_list(new_sequence.clone());
    assert_eq!(settings.sequence_list, new_sequence);
}
