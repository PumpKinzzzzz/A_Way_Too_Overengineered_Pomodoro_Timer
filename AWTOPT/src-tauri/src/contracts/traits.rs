// Traits: The common language for external tools from the warehouse
use super::dtos::*;

/// Trait for JSON serialization/deserialization (warehouse tool)
#[allow(clippy::wrong_self_convention)]
pub trait JsonTrait {
    fn to_json<T: serde::Serialize>(&self, value: &T) -> Result<String, String>;
    fn from_json<T: for<'de> serde::Deserialize<'de>>(&self, json: &str) -> Result<T, String>;
}

/// Trait for notification operations (warehouse tool)
pub trait NotifierTrait {
    fn notify(&self, title: &str, message: &str);
}

/// Trait for persistence operations (warehouse tool)
pub trait PersistenceTrait {
    fn save_settings(&self, settings: &SettingsResponse) -> Result<(), String>;
    fn load_settings(&self) -> Result<SettingsResponse, String>;
    fn save_session(&self, session: &SessionStatsResponse) -> Result<(), String>;
}

/// Trait for scheduling operations (warehouse tool)
pub trait SchedulerTrait {
    fn start_ticking(&mut self, interval_ms: u64);
    fn stop_ticking(&mut self);
}
