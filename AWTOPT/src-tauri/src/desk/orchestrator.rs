// The Foreman: Orchestrates the sequence of Workers
use crate::contracts::*;
use crate::workshop::{SessionWorker, SettingsWorker, TimerWorker};

/// PomodoroOrchestrator: Directs workers to accomplish complex tasks
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

    /// Handle start timer request
    pub fn handle_start_timer(
        &mut self,
        _request: StartTimerRequest,
    ) -> Result<TimerStatusResponse, String> {
        // Start the timer
        let response = self.timer_worker.start()?;

        // TODO: Notify user
        // TODO: Start scheduler

        Ok(response)
    }

    /// Handle pause timer request
    pub fn handle_pause_timer(
        &mut self,
        _request: PauseTimerRequest,
    ) -> Result<TimerStatusResponse, String> {
        // Pause the timer
        let response = self.timer_worker.pause()?;

        // TODO: Stop scheduler
        // TODO: Notify user

        Ok(response)
    }

    /// Handle resume timer request
    pub fn handle_resume_timer(
        &mut self,
        _request: ResumeTimerRequest,
    ) -> Result<TimerStatusResponse, String> {
        // Resume the timer
        let response = self.timer_worker.resume()?;

        // TODO: Start scheduler
        // TODO: Notify user

        Ok(response)
    }

    /// Handle reset timer request
    pub fn handle_reset_timer(
        &mut self,
        _request: ResetTimerRequest,
    ) -> Result<TimerStatusResponse, String> {
        // Reset the timer
        let response = self.timer_worker.reset()?;

        // TODO: Stop scheduler
        // TODO: Notify user

        Ok(response)
    }

    /// Handle tick (called by scheduler)
    pub fn handle_tick(&mut self) -> Result<TimerStatusResponse, String> {
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

    /// Handle update settings request
    pub fn handle_update_settings(
        &mut self,
        request: UpdateSettingsRequest,
    ) -> Result<SettingsResponse, String> {
        let response = self.settings_worker.update_settings(request)?;

        // TODO: Persist settings
        // TODO: Notify user

        Ok(response)
    }

    /// Handle get settings request
    pub fn handle_get_settings(&self) -> SettingsResponse {
        self.settings_worker.get_settings()
    }

    /// Handle get timer status request
    pub fn handle_get_timer_status(&self) -> TimerStatusResponse {
        self.timer_worker.get_status()
    }

    /// Handle get session stats request
    pub fn handle_get_session_stats(&self) -> SessionStatsResponse {
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
        let response = orchestrator
            .handle_start_timer(StartTimerRequest {})
            .unwrap();
        assert!(matches!(response.state, TimerStateDto::Running { .. }));

        // Pause timer
        let response = orchestrator
            .handle_pause_timer(PauseTimerRequest {})
            .unwrap();
        assert_eq!(response.state, TimerStateDto::Paused);

        // Resume timer
        let response = orchestrator
            .handle_resume_timer(ResumeTimerRequest {})
            .unwrap();
        assert!(matches!(response.state, TimerStateDto::Running { .. }));

        // Reset timer
        let response = orchestrator
            .handle_reset_timer(ResetTimerRequest {})
            .unwrap();
        assert_eq!(response.state, TimerStateDto::Idle);
    }

    #[test]
    fn test_orchestrator_settings() {
        let mut orchestrator = PomodoroOrchestrator::new();

        let request = UpdateSettingsRequest {
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
