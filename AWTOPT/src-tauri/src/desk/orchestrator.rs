use crate::contracts::*;
use crate::warehouse::TauriNotifierTool;
use crate::workshop::{
    NotifierWorker, PersistenceWorker, SessionWorker, SettingsWorker, TimerWorker,
};

pub struct PomodoroOrchestrator {
    timer_worker: TimerWorker,
    settings_worker: SettingsWorker,
    session_worker: SessionWorker,
    persistence_worker: PersistenceWorker,
    notifier_worker: NotifierWorker<TauriNotifierTool>,
}

impl PomodoroOrchestrator {
    pub fn new(
        timer_worker: TimerWorker,
        settings_worker: SettingsWorker,
        session_worker: SessionWorker,
        persistence_worker: PersistenceWorker,
        notifier_worker: NotifierWorker<TauriNotifierTool>,
    ) -> Self {
        Self {
            timer_worker,
            settings_worker,
            session_worker,
            persistence_worker,
            notifier_worker,
        }
    }

    fn save_current_state(&self) -> Result<(), String> {
        let state = AppSave {
            settings: self.settings_worker.get_settings(),
            timer_status: self.timer_worker.get_status(),
            session_stats: self.session_worker.get_stats(),
        };
        self.persistence_worker.save_app_state(&state)
    }

    pub fn handle_start_timer(&mut self) -> Result<TimerStatusDto, String> {
        let response = self.timer_worker.start()?;
        if let TimerStateDto::Running {
            sequence: SequenceType::Work,
        } = &response.state
        {
            self.notifier_worker.notify_work_started();
        }
        Ok(response)
    }

    pub fn handle_pause_timer(&mut self) -> Result<TimerStatusDto, String> {
        let response = self.timer_worker.pause()?;
        self.save_current_state()?;
        Ok(response)
    }

    pub fn handle_resume_timer(&mut self) -> Result<TimerStatusDto, String> {
        let response = self.timer_worker.resume()?;
        Ok(response)
    }

    pub fn handle_reset_timer(&mut self) -> Result<TimerStatusDto, String> {
        let response = self.timer_worker.reset()?;
        Ok(response)
    }

    pub fn handle_tick(&mut self) -> Result<TimerStatusDto, String> {
        let status_before = self.timer_worker.get_status();
        let response = self.timer_worker.tick()?;
        self.session_worker.record_time(1);

        if status_before.time_remaining == 1 && response.time_remaining != 0 {
            self.session_worker.increment_cycle();
            self.notifier_worker
                .notify_cycle_completed(response.current_cycle);

            if let TimerStateDto::Running { sequence } = &response.state {
                match sequence {
                    SequenceType::Work => self.notifier_worker.notify_work_started(),
                    SequenceType::ShortBreak => self.notifier_worker.notify_break_started(false),
                    SequenceType::LongBreak => self.notifier_worker.notify_break_started(true),
                }
            }
        }

        if matches!(response.state, TimerStateDto::Completed) {
            self.notifier_worker.notify_all_completed();
            self.save_current_state()?;
        }

        Ok(response)
    }

    pub fn handle_update_settings(
        &mut self,
        request: SettingsUpdateDto,
    ) -> Result<SettingsDto, String> {
        let response = self.settings_worker.update_settings(request)?;
        self.save_current_state()?;
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
