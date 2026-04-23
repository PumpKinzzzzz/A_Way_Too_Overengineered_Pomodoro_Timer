// Steve the Session: Specialized worker for session tracking operations
use crate::contracts::SessionStatsResponse;
use super::models::Session;

/// SessionWorker: Executes session tracking operations
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
