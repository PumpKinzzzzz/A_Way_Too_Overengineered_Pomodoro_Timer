pub struct Session {
    time_elapsed: u64,
    completed_cycles: usize,
    date: String,
}

impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Session {
            time_elapsed: 0,
            completed_cycles: 0,
            date: String::new(),
        }
    }

    pub fn update_time_elapsed(&mut self, time: u64) {
        self.time_elapsed += time;
    }

    pub fn increment_completed_cycles(&mut self) {
        self.completed_cycles += 1;
    }

    pub fn get_time_elapsed(&self) -> u64 {
        self.time_elapsed
    }

    pub fn get_completed_cycles(&self) -> usize {
        self.completed_cycles
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn set_date(&mut self, date: String) {
        self.date = date;
    }
}

#[test]
fn test_session() {
    let mut session = Session::new();
    assert_eq!(session.time_elapsed, 0);
    assert_eq!(session.completed_cycles, 0);

    session.update_time_elapsed(1500);
    assert_eq!(session.time_elapsed, 1500);

    session.increment_completed_cycles();
    assert_eq!(session.completed_cycles, 1);
}
