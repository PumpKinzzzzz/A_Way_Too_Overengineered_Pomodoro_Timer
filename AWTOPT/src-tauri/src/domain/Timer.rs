struct Timer {
    state: TimerState,
    current_cycle_index: usize,
    time_remaining: u64,
}

enum TimerState {
    Idle,
    Running(Sequence),  // carries the current phase
    Paused,
    Completed,
}

impl Timer {
    pub fn new() -> Self {
        Timer { state: TimerState::Idle, current_cycle_index: 0, time_remaining: 0 }
    }

    pub fn start(&mut self, settings: &Settings) {
        if let TimerState::Idle = self.state {
            self.current_cycle_index = 0;
            self.time_remaining = settings.workDuration * 60;
            self.state = TimerState::Running(Sequence::Work);
        }
    }

    pub fn pause(&mut self) {
        if let TimerState::Running(Sequence) | TimerState::RunningBreak | TimerState::RunningBreakLong = self.state {
            self.state = TimerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let TimerState::Paused = self.state {
            match self.current_cycle_index {
                0 | 2 => self.state = TimerState::Running(Sequence::Work),
                1 => self.state = TimerState::Running(Sequence::Break),
                3 => self.state = TimerState::Running(Sequence::BreakLong),
                _ => (),
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
        self.current_cycle_index = 0;
        self.time_remaining = 0;
    }
}

#[test]
fn test_timer() {
    let mut timer = Timer::new();
    let settings = Settings::new();
    timer.start(&settings);
    assert_eq!(timer.state, TimerState::Running(Sequence::Work));
    timer.pause();
    assert_eq!(timer.state, TimerState::Paused);
    timer.resume();
    assert_eq!(timer.state, TimerState::Running(Sequence::Work));
    timer.reset();
    assert_eq!(timer.state, TimerState::Idle);
}
