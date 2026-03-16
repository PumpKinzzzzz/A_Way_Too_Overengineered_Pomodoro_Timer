use crate::domain::timer::Timer;
use crate::domain::session::Session;
use crate::domain::settings::Settings;
use crate::ports::outbound::notify::NotifyPort;
use crate::ports::outbound::save::Save;
use crate::ports::outbound::schedule::Scheduler;

pub trait TimerPort {
    fn start(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn reset(&mut self);
    fn tick(&mut self);
}

pub struct TimerService<N: NotifyPort, S: Save, Sc: Scheduler> {
    timer: Timer,
    session: Session,
    notifier: N,
    saver: S,
    scheduler: Sc,
}

impl<N: NotifyPort, S: Save, Sc: Scheduler> TimerService<N, S, Sc> {
    pub fn new(settings: &Settings, notifier: N, saver: S, scheduler: Sc) -> Self {
        TimerService {
            timer: Timer::new(settings),
            session: Session::new(),
            notifier,
            saver,
            scheduler,
        }
    }
}

impl<N: NotifyPort, S: Save, Sc: Scheduler> TimerPort for TimerService<N, S, Sc> {
    fn start(&mut self) {
        self.timer.start();
        self.scheduler.start_ticking();
        self.notifier.notify("Timer", "Timer started!");
        
    }

    fn pause(&mut self) {
        self.timer.pause();
        self.scheduler.stop_ticking();
        self.notifier.notify("Timer", "Timer paused");
    }

    fn resume(&mut self) {
        self.timer.resume();
        self.scheduler.start_ticking();
        self.notifier.notify("Timer", "Timer resumed");
    }

    fn reset(&mut self) {
        self.timer.reset();
        self.scheduler.stop_ticking();
        self.notifier.notify("Timer", "Timer reset");
    }

    fn tick(&mut self) {
        self.timer.tick();
        
        // TODO: Vérifier si un cycle vient de finir
    }
}