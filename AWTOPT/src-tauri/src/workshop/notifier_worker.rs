use crate::contracts::NotifierTrait;

pub struct NotifierWorker<T: NotifierTrait> {
    tool: T,
}

impl<T: NotifierTrait> NotifierWorker<T> {
    pub fn new(tool: T) -> Self {
        Self { tool }
    }

    pub fn notify_work_started(&self) {
        self.tool.notify("Pomodoro", "Work session started!");
    }

    pub fn notify_work_completed(&self) {
        self.tool.notify("Pomodoro", "Work session completed!");
    }

    pub fn notify_break_started(&self, is_long: bool) {
        let message = if is_long {
            "Long break started!"
        } else {
            "Short break started!"
        };
        self.tool.notify("Pomodoro", message);
    }

    pub fn notify_break_completed(&self) {
        self.tool.notify("Pomodoro", "Break completed!");
    }

    pub fn notify_cycle_completed(&self, cycles: usize) {
        let message = format!("Cycle {} completed!", cycles);
        self.tool.notify("Pomodoro", &message);
    }

    pub fn notify_all_completed(&self) {
        self.tool.notify("Pomodoro", "All sessions completed!");
    }
}

impl<T: NotifierTrait> NotifierTrait for NotifierWorker<T> {
    fn notify(&self, title: &str, message: &str) {
        self.tool.notify(title, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockNotifier {
        notifications: std::cell::RefCell<Vec<(String, String)>>,
    }

    impl MockNotifier {
        fn new() -> Self {
            Self {
                notifications: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn get_notifications(&self) -> Vec<(String, String)> {
            self.notifications.borrow().clone()
        }
    }

    impl NotifierTrait for MockNotifier {
        fn notify(&self, title: &str, message: &str) {
            self.notifications
                .borrow_mut()
                .push((title.to_string(), message.to_string()));
        }
    }

    #[test]
    fn test_notifier_worker() {
        let mock = MockNotifier::new();
        let worker = NotifierWorker::new(mock);

        worker.notify_work_started();
        worker.notify_work_completed();
        worker.notify_cycle_completed(3);

        let notifications = worker.tool.get_notifications();
        assert_eq!(notifications.len(), 3);
        assert_eq!(notifications[0], ("Pomodoro".to_string(), "Work session started!".to_string()));
        assert_eq!(notifications[1], ("Pomodoro".to_string(), "Work session completed!".to_string()));
        assert_eq!(notifications[2], ("Pomodoro".to_string(), "Cycle 3 completed!".to_string()));
    }
}
