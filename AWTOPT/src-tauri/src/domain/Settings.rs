enum Sequence {
    Work,
    ShortBreak,
    LongBreak,
}

struct Settings {
    workDuration: u64,
    shortBreakDuration: u64,
    longBreakDuration: u64,
    autoStartBreaks: bool,
    sequenceList: Vec<Sequence>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            workDuration: 25,
            shortBreakDuration: 5,
            longBreakDuration: 15,
            autoStartBreaks: true,
            sequenceList: vec![Sequence::Work, Sequence::ShortBreak, Sequence::Work, Sequence::LongBreak],
        }
    }

    pub fn updateWorkDuration(&mut self, duration: u64) {
        self.workDuration = duration;
    }

    pub fn updateShortBreakDuration(&mut self, duration: u64) {
        self.shortBreakDuration = duration;
    }

    pub fn updateLongBreakDuration(&mut self, duration: u64) {
        self.longBreakDuration = duration;
    }

    pub fn toggleAutoStartBreaks(&mut self) {
        self.autoStartBreaks = !self.autoStartBreaks;
    }

    pub fn updateSequenceList(&mut self, sequenceList: Vec<Sequence>) {
        self.sequenceList = sequenceList;
    }
}

#[test]
fn test_settings() {
    let mut settings = Settings::new();
    assert_eq!(settings.workDuration, 25);
    settings.updateWorkDuration(30);
    assert_eq!(settings.workDuration, 30);
    settings.toggleAutoStartBreaks();
    assert_eq!(settings.autoStartBreaks, false);
}
