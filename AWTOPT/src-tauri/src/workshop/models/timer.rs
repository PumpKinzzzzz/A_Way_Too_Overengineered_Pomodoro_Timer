use super::settings::Settings;
use super::types::Sequence;

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
