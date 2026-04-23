use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SequenceType {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerStateDto {
    Idle,
    Running { sequence: SequenceType },
    Paused,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimerStatusDto {
    pub state: TimerStateDto,
    pub time_remaining: u64,
    pub current_cycle: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsUpdateDto {
    pub work_duration: Option<u64>,
    pub short_break_duration: Option<u64>,
    pub long_break_duration: Option<u64>,
    pub auto_start_breaks: Option<bool>,
    pub sequence_list: Option<Vec<SequenceType>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsDto {
    pub work_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub auto_start_breaks: bool,
    pub sequence_list: Vec<SequenceType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionStatsDto {
    pub time_elapsed: u64,
    pub completed_cycles: usize,
    pub date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessDto {
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorDto {
    pub error: String,
}
