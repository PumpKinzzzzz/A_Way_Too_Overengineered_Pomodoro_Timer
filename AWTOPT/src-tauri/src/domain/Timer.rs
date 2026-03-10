struct Timer {
    state: TimerState,
    currentCycleIndex: usize,
    timeRemaining: u64,
}

enum TimerState {
    Idle,
    RunningWork,
    RunningBreak,
    RunningBreakLong,
    Paused,
    Completed,
}

impl Timer {
    pub fn new() -> Self {
        Timer { state: TimerState::Idle, currentCycleIndex: 0, timeRemaining: 0, startedAt: Timestamp::now() }
    }

    pub fn start(&mut self, settings: &Settings) {
        if let TimerState::Idle = self.state {
            self.currentCycleIndex = 0;
            self.timeRemaining = settings.workDuration * 60;
            self.state = TimerState::RunningWork;
        }
    }

    pub fn pause(&mut self) {
        if let TimerState::RunningWork | TimerState::RunningBreak | TimerState::RunningBreakLong = self.state {
            self.state = TimerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let TimerState::Paused = self.state {
            match self.currentCycleIndex {
                0 | 2 => self.state = TimerState::RunningWork,
                1 => self.state = TimerState::RunningBreak,
                3 => self.state = TimerState::RunningBreakLong,
                _ => (),
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
        self.currentCycleIndex = 0;
        self.timeRemaining = 0;
    }
}

#[test]
fn test_timer() {
    let mut timer = Timer::new();
    let settings = Settings::new();
    timer.start(&settings);
    assert_eq!(timer.state, TimerState::RunningWork);
    timer.pause();
    assert_eq!(timer.state, TimerState::Paused);
    timer.resume();
    assert_eq!(timer.state, TimerState::RunningWork);
    timer.reset();
    assert_eq!(timer.state, TimerState::Idle);
}
