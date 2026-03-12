pub trait TimerPort {
    fn start(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn reset(&mut self);
    fn tick(&mut self);
}