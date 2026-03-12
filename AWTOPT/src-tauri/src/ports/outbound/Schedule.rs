pub trait Scheduler {
    fn start_ticking(&self);
    fn stop_ticking(&self);
}