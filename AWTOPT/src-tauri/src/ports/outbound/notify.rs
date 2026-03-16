pub trait NotifyPort {
    fn notify(&self, title: &str, message: &str);
}