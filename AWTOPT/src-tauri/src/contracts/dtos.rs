// DTOs: The blueprints that circulate between layers
use serde::{Deserialize, Serialize};

/// Sequence types for the Pomodoro timer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SequenceType {
    Work,
    ShortBreak,
    LongBreak,
}

/// Timer state representation
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerStateDto {
    Idle,
    Running { sequence: SequenceType },
    Paused,
    Completed,
}

/// Request to start the timer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartTimerRequest {}

/// Request to pause the timer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PauseTimerRequest {}

/// Request to resume the timer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResumeTimerRequest {}

/// Request to reset the timer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResetTimerRequest {}

/// Response containing timer status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimerStatusResponse {
    pub state: TimerStateDto,
    pub time_remaining: u64,
    pub current_cycle: usize,
}

/// Request to update settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    pub work_duration: Option<u64>,
    pub short_break_duration: Option<u64>,
    pub long_break_duration: Option<u64>,
    pub auto_start_breaks: Option<bool>,
    pub sequence_list: Option<Vec<SequenceType>>,
}

/// Response containing current settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub work_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub auto_start_breaks: bool,
    pub sequence_list: Vec<SequenceType>,
}

/// Response containing session statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionStatsResponse {
    pub time_elapsed: u64,
    pub completed_cycles: usize,
    pub date: String,
}

/// Generic success response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub message: String,
}

/// Generic error response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
