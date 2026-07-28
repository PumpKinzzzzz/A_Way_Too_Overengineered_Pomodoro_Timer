use crate::contracts::{AppSave, PersistenceTrait};

pub struct PersistenceWorker<T: PersistenceTrait> {
    tool: T,
}

impl<T: PersistenceTrait> PersistenceWorker<T> {
    pub fn new(tool: T) -> Self {
        Self { tool }
    }

    pub fn save_app_state(&self, state: &AppSave) -> Result<(), String> {
        self.tool.save_state(state)
    }

    pub fn load_app_state(&self) -> Result<AppSave, String> {
        self.tool.load_state()
    }

    pub fn has_saved_state(&self) -> bool {
        self.tool.state_exists()
    }

    pub fn clear_saved_state(&self) -> Result<(), String> {
        self.tool.delete_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{SequenceType, SessionStatsDto, SettingsDto};
    use std::cell::RefCell;

    struct MockPersistence {
        state: RefCell<Option<AppSave>>,
    }

    impl MockPersistence {
        fn new() -> Self {
            Self {
                state: RefCell::new(None),
            }
        }
    }

    impl PersistenceTrait for MockPersistence {
        fn save_state(&self, state: &AppSave) -> Result<(), String> {
            *self.state.borrow_mut() = Some(state.clone());
            Ok(())
        }

        fn load_state(&self) -> Result<AppSave, String> {
            self.state
                .borrow()
                .clone()
                .ok_or_else(|| "No saved state".to_string())
        }

        fn state_exists(&self) -> bool {
            self.state.borrow().is_some()
        }

        fn delete_state(&self) -> Result<(), String> {
            *self.state.borrow_mut() = None;
            Ok(())
        }
    }

    fn create_test_state() -> AppSave {
        AppSave {
            settings: SettingsDto {
                work_duration: 25,
                short_break_duration: 5,
                long_break_duration: 15,
                auto_start_breaks: true,
                sequence_list: vec![
                    SequenceType::Work,
                    SequenceType::ShortBreak,
                    SequenceType::Work,
                    SequenceType::LongBreak,
                ],
            },
            session_stats: SessionStatsDto {
                time_elapsed: 0,
                completed_cycles: 0,
                date: "2026-05-11".to_string(),
            },
        }
    }

    #[test]
    fn test_persistence_worker_lifecycle() {
        let worker = PersistenceWorker::new(MockPersistence::new());

        assert!(!worker.has_saved_state());

        let state = create_test_state();
        worker.save_app_state(&state).unwrap();

        assert!(worker.has_saved_state());
        let loaded = worker.load_app_state().unwrap();
        assert_eq!(loaded.settings.work_duration, state.settings.work_duration);

        worker.clear_saved_state().unwrap();
        assert!(!worker.has_saved_state());
    }
}
