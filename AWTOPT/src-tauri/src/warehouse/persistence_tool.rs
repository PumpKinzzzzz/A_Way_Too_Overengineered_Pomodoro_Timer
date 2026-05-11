use crate::contracts::AppState;
use crate::warehouse::SerdeJsonTool;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// Tool for persisting application state to disk
///
/// Saves and loads the complete application state (settings, timer, stats)
/// to a JSON file in the application's data directory.
pub struct PersistenceTool {
    json_tool: SerdeJsonTool,
    file_path: PathBuf,
}

impl PersistenceTool {
    /// Create a new PersistenceTool
    ///
    /// # Arguments
    /// * `app_handle` - Tauri app handle to get the app data directory
    ///
    /// # Errors
    /// Returns error if the app data directory cannot be resolved
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        // Get the app data directory from Tauri
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        // Ensure the directory exists
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;

        // Build the file path
        let file_path = app_data_dir.join("pomodoro_state.json");

        Ok(Self {
            json_tool: SerdeJsonTool::new(),
            file_path,
        })
    }

    /// Save application state to disk
    ///
    /// # Arguments
    /// * `state` - The application state to save
    ///
    /// # Errors
    /// Returns error if serialization or file write fails
    pub fn save_state(&self, state: &AppState) -> Result<(), String> {
        // Serialize to JSON
        let json = self.json_tool.to_json(state)?;

        // Write to file
        fs::write(&self.file_path, json)
            .map_err(|e| format!("Failed to write state file: {}", e))?;

        Ok(())
    }

    /// Load application state from disk
    ///
    /// # Errors
    /// Returns error if file read or deserialization fails
    pub fn load_state(&self) -> Result<AppState, String> {
        // Read file
        let json = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read state file: {}", e))?;

        // Deserialize from JSON
        let state = self.json_tool.from_json(&json)?;

        Ok(state)
    }

    /// Check if a saved state file exists
    pub fn state_exists(&self) -> bool {
        self.file_path.exists()
    }

    /// Get the path where state is stored (useful for debugging)
    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }

    /// Delete the saved state file
    ///
    /// # Errors
    /// Returns error if file deletion fails
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

    fn create_test_state() -> AppState {
        AppState {
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
        let json_tool = SerdeJsonTool::new();
        let state = create_test_state();

        let json = json_tool.to_json(&state).unwrap();
        let restored: AppState = json_tool.from_json(&json).unwrap();

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
