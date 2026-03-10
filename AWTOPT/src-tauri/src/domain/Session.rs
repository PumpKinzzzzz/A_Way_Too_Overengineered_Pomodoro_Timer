struct Session {
    timeElapsed: u64,
    completedCycles: usize,
    date: String,
}

impl Session {
    pub fn new() -> Self {
        Session { timeElapsed: 0, completedCycles: 0, date: Timestamp::now().to_string() }
    }

    pub fn updateTimeElapsed(&mut self, time: u64) {
        self.timeElapsed += time;
    }

    pub fn incrementCompletedCycles(&mut self) {
        self.completedCycles += 1;
    }
}

#[test]
fn test_session() {
    let mut session = Session::new();
    assert_eq!(session.timeElapsed, 0);
    session.updateTimeElapsed(1500);
    assert_eq!(session.timeElapsed, 1500);
    assert_eq!(session.completedCycles, 0);
    session.incrementCompletedCycles();
    assert_eq!(session.completedCycles, 1);
}