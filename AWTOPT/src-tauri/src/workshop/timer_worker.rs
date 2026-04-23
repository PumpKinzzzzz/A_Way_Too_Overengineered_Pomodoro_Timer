// Peter the Timer: Specialized worker for timer operations
use super::models::{Sequence, Settings, Timer, TimerState};
use crate::contracts::{SequenceType, TimerStateDto, TimerStatusResponse};

/// TimerWorker: Executes timer-related operations
pub struct TimerWorker {
    timer: Timer,
    current_cycle_index: usize,
}

impl TimerWorker {
    pub fn new(settings: &Settings) -> Self {
        Self {
            timer: Timer::new(settings),
            current_cycle_index: 0,
        }
    }

    /// Convert domain TimerState to DTO
    fn state_to_dto(state: &TimerState) -> TimerStateDto {
        match state {
            TimerState::Idle => TimerStateDto::Idle,
            TimerState::Running(seq) => TimerStateDto::Running {
                sequence: Self::sequence_to_dto(seq),
            },
            TimerState::Paused => TimerStateDto::Paused,
            TimerState::Completed => TimerStateDto::Completed,
        }
    }

    /// Convert domain Sequence to DTO
    fn sequence_to_dto(seq: &Sequence) -> SequenceType {
        match seq {
            Sequence::Work => SequenceType::Work,
            Sequence::ShortBreak => SequenceType::ShortBreak,
            Sequence::LongBreak => SequenceType::LongBreak,
        }
    }

    pub fn start(&mut self) -> Result<TimerStatusResponse, String> {
        self.timer.start();
        self.current_cycle_index = 0;
        Ok(self.get_status())
    }

    pub fn pause(&mut self) -> Result<TimerStatusResponse, String> {
        self.timer.pause();
        Ok(self.get_status())
    }

    pub fn resume(&mut self) -> Result<TimerStatusResponse, String> {
        self.timer.resume();
        Ok(self.get_status())
    }

    pub fn reset(&mut self) -> Result<TimerStatusResponse, String> {
        self.timer.reset();
        self.current_cycle_index = 0;
        Ok(self.get_status())
    }

    pub fn tick(&mut self) -> Result<TimerStatusResponse, String> {
        // Check if we're completing a cycle before ticking
        let was_running = matches!(self.timer.get_state(), TimerState::Running(_));
        let time_before = self.timer.get_time_remaining();

        self.timer.tick();

        // If time went from 1 to 0, we completed a cycle
        if was_running && time_before == 1 && self.timer.get_time_remaining() != 0 {
            self.current_cycle_index += 1;
        }

        Ok(self.get_status())
    }

    pub fn get_status(&self) -> TimerStatusResponse {
        TimerStatusResponse {
            state: Self::state_to_dto(self.timer.get_state()),
            time_remaining: self.timer.get_time_remaining(),
            current_cycle: self.current_cycle_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_worker_lifecycle() {
        let settings = Settings::new();
        let mut worker = TimerWorker::new(&settings);

        // Initial state
        let status = worker.get_status();
        assert_eq!(status.state, TimerStateDto::Idle);

        // Start
        let status = worker.start().unwrap();
        assert!(matches!(
            status.state,
            TimerStateDto::Running {
                sequence: SequenceType::Work
            }
        ));

        // Pause
        let status = worker.pause().unwrap();
        assert_eq!(status.state, TimerStateDto::Paused);

        // Resume
        let status = worker.resume().unwrap();
        assert!(matches!(status.state, TimerStateDto::Running { .. }));

        // Reset
        let status = worker.reset().unwrap();
        assert_eq!(status.state, TimerStateDto::Idle);
    }
}
