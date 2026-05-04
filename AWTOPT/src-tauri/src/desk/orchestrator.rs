use crate::contracts::*;
use crate::workshop::{SessionWorker, SettingsWorker, TimerWorker};

pub struct PomodoroOrchestrator {
    timer_worker: TimerWorker,
    settings_worker: SettingsWorker,
    session_worker: SessionWorker,
}

impl PomodoroOrchestrator {
    pub fn new() -> Self {
        let settings_worker = SettingsWorker::new();
        let timer_worker = TimerWorker::new(settings_worker.get_domain_settings());
        let session_worker = SessionWorker::new();

        Self {
            timer_worker,
            settings_worker,
            session_worker,
        }
    }

    pub fn handle_start_timer(&mut self) -> Result<TimerStatusDto, String> {
        // Start the timer
        let response = self.timer_worker.start()?;

        // TODO: Notify user
        // TODO: Start scheduler

        Ok(response)
    }

    pub fn handle_pause_timer(&mut self) -> Result<TimerStatusDto, String> {
        // Pause the timer
        let response = self.timer_worker.pause()?;

        // TODO: Stop scheduler
        // TODO: Notify user

        Ok(response)
    }

    pub fn handle_resume_timer(&mut self) -> Result<TimerStatusDto, String> {
        // Resume the timer
        let response = self.timer_worker.resume()?;

        // TODO: Start scheduler
        // TODO: Notify user

        Ok(response)
    }

    pub fn handle_reset_timer(&mut self) -> Result<TimerStatusDto, String> {
        // Reset the timer
        let response = self.timer_worker.reset()?;

        // TODO: Stop scheduler
        // TODO: Notify user

        Ok(response)
    }

    pub fn handle_tick(&mut self) -> Result<TimerStatusDto, String> {
        // Get status before tick
        let status_before = self.timer_worker.get_status();

        // Tick the timer
        let response = self.timer_worker.tick()?;

        // Record elapsed time in session
        self.session_worker.record_time(1);

        // Check if we completed a cycle (time went to 0 and moved to next sequence)
        if status_before.time_remaining == 1 && response.time_remaining != 0 {
            self.session_worker.increment_cycle();
            // TODO: Notify user of cycle completion
        }

        // Check if timer completed
        if matches!(response.state, TimerStateDto::Completed) {
            // TODO: Stop scheduler
            // TODO: Notify user of completion
        }

        Ok(response)
    }

    pub fn handle_update_settings(
        &mut self,
        request: SettingsUpdateDto,
    ) -> Result<SettingsDto, String> {
        let response = self.settings_worker.update_settings(request)?;

        // TODO: Persist settings
        // TODO: Notify user

        Ok(response)
    }

    pub fn handle_get_settings(&self) -> SettingsDto {
        self.settings_worker.get_settings()
    }

    pub fn handle_get_timer_status(&self) -> TimerStatusDto {
        self.timer_worker.get_status()
    }

    pub fn handle_get_session_stats(&self) -> SessionStatsDto {
        self.session_worker.get_stats()
    }
}

impl Default for PomodoroOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_timer_lifecycle() {
        let mut orchestrator = PomodoroOrchestrator::new();

        // Start timer
        let response = orchestrator.handle_start_timer().unwrap();
        assert!(matches!(response.state, TimerStateDto::Running { .. }));

        // Pause timer
        let response = orchestrator.handle_pause_timer().unwrap();
        assert_eq!(response.state, TimerStateDto::Paused);

        // Resume timer
        let response = orchestrator.handle_resume_timer().unwrap();
        assert!(matches!(response.state, TimerStateDto::Running { .. }));

        // Reset timer
        let response = orchestrator.handle_reset_timer().unwrap();
        assert_eq!(response.state, TimerStateDto::Idle);
    }

    #[test]
    fn test_orchestrator_settings() {
        let mut orchestrator = PomodoroOrchestrator::new();

        let request = SettingsUpdateDto {
            work_duration: Some(30),
            short_break_duration: None,
            long_break_duration: None,
            auto_start_breaks: None,
            sequence_list: None,
        };

        let response = orchestrator.handle_update_settings(request).unwrap();
        assert_eq!(response.work_duration, 30);
    }
}
