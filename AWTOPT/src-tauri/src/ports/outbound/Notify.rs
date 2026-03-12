pub trait NotifyPort {
    fn notify_sequence_finished(&self, title: &str, message: &str);
    fn notify_timer_completed(&self, title: &str, message: &str);
}