// Peter the Timer: Specialized worker for timer operations
use super::types::Sequence;
use crate::contracts::{SequenceType, TimerStateDto, TimerStatusResponse};

// ============================================================================
// Internal Timer Model
// ============================================================================

pub struct Timer {
    state: TimerState,
    current_cycle_index: usize,
    time_remaining: u64,
    sequence_list: Vec<Sequence>,
    durations: (u64, u64, u64), // (work_duration, short_break_duration, long_break_duration)
}

pub enum TimerState {
    Idle,
    Running(Sequence), // work, short break, or long break
    Paused,
    Completed,
}

impl Timer {
    pub fn new(settings: &Settings) -> Self {
        Timer {
            state: TimerState::Idle,
            current_cycle_index: 0,
            time_remaining: 0,
            sequence_list: settings.get_sequence_list().clone(),
            durations: (
                settings.get_work_duration(),
                settings.get_short_break_duration(),
                settings.get_long_break_duration(),
            ),
        }
    }

    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    pub fn get_time_remaining(&self) -> u64 {
        self.time_remaining
    }

    pub fn start(&mut self) {
        if let TimerState::Idle = self.state {
            self.current_cycle_index = 0;
            self.time_remaining = self.durations.0 * 60;
            self.state = TimerState::Running(self.sequence_list[self.current_cycle_index]);
        }
    }

    pub fn pause(&mut self) {
        if let TimerState::Running(_) = self.state {
            self.state = TimerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let TimerState::Paused = self.state {
            let current_sequence = self.sequence_list[self.current_cycle_index];
            self.state = TimerState::Running(current_sequence);
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
        self.current_cycle_index = 0;
        self.time_remaining = 0;
    }

    pub fn tick(&mut self) {
        if let TimerState::Running(_) = self.state {
            if self.time_remaining > 0 {
                self.time_remaining -= 1;
            }

            if self.time_remaining == 0 {
                self.current_cycle_index += 1;

                if self.current_cycle_index >= self.sequence_list.len() {
                    self.state = TimerState::Completed;
                    return;
                }

                let next = self.sequence_list[self.current_cycle_index];
                self.time_remaining = match next {
                    Sequence::Work => self.durations.0 * 60,
                    Sequence::ShortBreak => self.durations.1 * 60,
                    Sequence::LongBreak => self.durations.2 * 60,
                };
                self.state = TimerState::Running(next);
            }
        }
    }
}

// ============================================================================
// Settings (needed by Timer)
// ============================================================================

pub struct Settings {
    work_duration: u64,
    short_break_duration: u64,
    long_break_duration: u64,
    auto_start_breaks: bool,
    sequence_list: Vec<Sequence>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            work_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            auto_start_breaks: true,
            sequence_list: vec![
                Sequence::Work,
                Sequence::ShortBreak,
                Sequence::Work,
                Sequence::LongBreak,
            ],
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

    pub fn get_work_duration(&self) -> u64 {
        self.work_duration
    }

    pub fn get_short_break_duration(&self) -> u64 {
        self.short_break_duration
    }

    pub fn get_long_break_duration(&self) -> u64 {
        self.long_break_duration
    }

    pub fn is_auto_start_breaks(&self) -> bool {
        self.auto_start_breaks
    }

    pub fn get_sequence_list(&self) -> &Vec<Sequence> {
        &self.sequence_list
    }
}

// ============================================================================
// TimerWorker - The Worker that uses Timer
// ============================================================================

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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let settings = Settings::new();
        let mut timer = Timer::new(&settings);

        assert!(matches!(timer.state, TimerState::Idle));

        timer.start();
        assert!(matches!(timer.state, TimerState::Running(Sequence::Work)));
        assert_eq!(timer.time_remaining, settings.get_work_duration() * 60);

        timer.pause();
        assert!(matches!(timer.state, TimerState::Paused));

        timer.resume();
        assert!(matches!(timer.state, TimerState::Running(Sequence::Work)));

        timer.reset();
        assert!(matches!(timer.state, TimerState::Idle));
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

        assert!(settings.auto_start_breaks);
        settings.toggle_auto_start_breaks();
        assert!(!settings.auto_start_breaks);

        let new_sequence = vec![Sequence::Work, Sequence::Work, Sequence::ShortBreak];
        settings.update_sequence_list(new_sequence.clone());
        assert_eq!(settings.sequence_list, new_sequence);
    }

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
