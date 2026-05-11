use crate::contracts::AppSave;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

pub struct PersistenceTool {
    file_path: PathBuf,
}

impl PersistenceTool {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;

        let file_path = app_data_dir.join("pomodoro_state.json");

        Ok(Self { file_path })
    }

    pub fn save_state(&self, state: &AppSave) -> Result<(), String> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        fs::write(&self.file_path, json)
            .map_err(|e| format!("Failed to write state file: {}", e))?;
        Ok(())
    }

    pub fn load_state(&self) -> Result<AppSave, String> {
        let json = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read state file: {}", e))?;
        let state = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize state: {}", e))?;
        Ok(state)
    }

    pub fn state_exists(&self) -> bool {
        self.file_path.exists()
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn delete_state(&self) -> Result<(), String> {
        if self.state_exists() {
            fs::remove_file(&self.file_path)
                .map_err(|e| format!("Failed to delete state file: {}", e))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{
        SequenceType, SessionStatsDto, SettingsDto, TimerStateDto, TimerStatusDto,
    };

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
            timer_status: TimerStatusDto {
                state: TimerStateDto::Idle,
                time_remaining: 0,
                current_cycle: 0,
            },
            session_stats: SessionStatsDto {
                time_elapsed: 0,
                completed_cycles: 0,
                date: "2026-05-11".to_string(),
            },
        }
    }

    #[test]
    fn test_state_serialization() {
        let state = create_test_state();

        let json = serde_json::to_string(&state).unwrap();
        let restored: AppSave = serde_json::from_str(&json).unwrap();

        assert_eq!(
            state.settings.work_duration,
            restored.settings.work_duration
        );
        assert_eq!(
            state.timer_status.time_remaining,
            restored.timer_status.time_remaining
        );
        assert_eq!(
            state.session_stats.completed_cycles,
            restored.session_stats.completed_cycles
        );
    }

    // Note: Tests requiring AppHandle need integration tests with Tauri runtime
}
