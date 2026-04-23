use super::dtos::*;

#[allow(clippy::wrong_self_convention)]
pub trait JsonTrait {
    fn to_json<T: serde::Serialize>(&self, value: &T) -> Result<String, String>;
    fn from_json<T: for<'de> serde::Deserialize<'de>>(&self, json: &str) -> Result<T, String>;
}

pub trait NotifierTrait {
    fn notify(&self, title: &str, message: &str);
}

pub trait PersistenceTrait {
    fn save_settings(&self, settings: &SettingsDto) -> Result<(), String>;
    fn load_settings(&self) -> Result<SettingsDto, String>;
    fn save_session(&self, session: &SessionStatsDto) -> Result<(), String>;
}

pub trait SchedulerTrait {
    fn start_ticking(&mut self, interval_ms: u64);
    fn stop_ticking(&mut self);
}
