use crate::contracts::SessionStatsResponse;

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

pub struct SessionWorker {
    session: Session,
}

impl SessionWorker {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
        }
    }

    pub fn with_session(session: Session) -> Self {
        Self { session }
    }

    pub fn record_time(&mut self, seconds: u64) {
        self.session.update_time_elapsed(seconds);
    }

    pub fn increment_cycle(&mut self) {
        self.session.increment_completed_cycles();
    }

    pub fn get_stats(&self) -> SessionStatsResponse {
        SessionStatsResponse {
            time_elapsed: self.session.get_time_elapsed(),
            completed_cycles: self.session.get_completed_cycles(),
            date: self.session.get_date().to_string(),
        }
    }

    pub fn reset(&mut self) {
        self.session = Session::new();
    }
}

impl Default for SessionWorker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_session_worker() {
        let mut worker = SessionWorker::new();

        // Record time
        worker.record_time(1500);
        let stats = worker.get_stats();
        assert_eq!(stats.time_elapsed, 1500);

        // Increment cycles
        worker.increment_cycle();
        let stats = worker.get_stats();
        assert_eq!(stats.completed_cycles, 1);

        // Reset
        worker.reset();
        let stats = worker.get_stats();
        assert_eq!(stats.time_elapsed, 0);
        assert_eq!(stats.completed_cycles, 0);
    }
}
