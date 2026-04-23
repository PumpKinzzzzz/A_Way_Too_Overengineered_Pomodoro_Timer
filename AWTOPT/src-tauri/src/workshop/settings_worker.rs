// Mary the Settings: Specialized worker for settings operations
use crate::contracts::{SequenceType, SettingsResponse, UpdateSettingsRequest};
use super::models::{Settings, Sequence};

/// SettingsWorker: Executes settings-related operations
pub struct SettingsWorker {
    settings: Settings,
}

impl SettingsWorker {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn get_domain_settings(&self) -> &Settings {
        &self.settings
    }

    /// Convert domain Sequence to DTO
    fn sequence_to_dto(seq: &Sequence) -> SequenceType {
        match seq {
            Sequence::Work => SequenceType::Work,
            Sequence::ShortBreak => SequenceType::ShortBreak,
            Sequence::LongBreak => SequenceType::LongBreak,
        }
    }

    /// Convert DTO Sequence to domain
    fn dto_to_sequence(seq: &SequenceType) -> Sequence {
        match seq {
            SequenceType::Work => Sequence::Work,
            SequenceType::ShortBreak => Sequence::ShortBreak,
            SequenceType::LongBreak => Sequence::LongBreak,
        }
    }

    pub fn update_settings(&mut self, request: UpdateSettingsRequest) -> Result<SettingsResponse, String> {
        // Update work duration if provided
        if let Some(duration) = request.work_duration {
            self.settings.update_work_duration(duration);
        }

        // Update short break duration if provided
        if let Some(duration) = request.short_break_duration {
            self.settings.update_short_break_duration(duration);
        }

        // Update long break duration if provided
        if let Some(duration) = request.long_break_duration {
            self.settings.update_long_break_duration(duration);
        }

        // Toggle auto start breaks if provided
        if let Some(auto_start) = request.auto_start_breaks {
            if auto_start != self.settings.is_auto_start_breaks() {
                self.settings.toggle_auto_start_breaks();
            }
        }

        // Update sequence list if provided
        if let Some(sequence_list) = request.sequence_list {
            let domain_sequences: Vec<Sequence> = sequence_list
                .iter()
                .map(Self::dto_to_sequence)
                .collect();
            self.settings.update_sequence_list(domain_sequences);
        }

        Ok(self.get_settings())
    }

    pub fn get_settings(&self) -> SettingsResponse {
        SettingsResponse {
            work_duration: self.settings.get_work_duration(),
            short_break_duration: self.settings.get_short_break_duration(),
            long_break_duration: self.settings.get_long_break_duration(),
            auto_start_breaks: self.settings.is_auto_start_breaks(),
            sequence_list: self
                .settings
                .get_sequence_list()
                .iter()
                .map(Self::sequence_to_dto)
                .collect(),
        }
    }
}

impl Default for SettingsWorker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_worker_update() {
        let mut worker = SettingsWorker::new();

        let request = UpdateSettingsRequest {
            work_duration: Some(30),
            short_break_duration: Some(10),
            long_break_duration: None,
            auto_start_breaks: Some(false),
            sequence_list: None,
        };

        let response = worker.update_settings(request).unwrap();
        assert_eq!(response.work_duration, 30);
        assert_eq!(response.short_break_duration, 10);
        assert_eq!(response.long_break_duration, 15); // unchanged
        assert!(!response.auto_start_breaks);
    }

    #[test]
    fn test_settings_worker_get() {
        let worker = SettingsWorker::new();
        let settings = worker.get_settings();
        
        assert_eq!(settings.work_duration, 25);
        assert_eq!(settings.short_break_duration, 5);
        assert_eq!(settings.long_break_duration, 15);
        assert!(settings.auto_start_breaks);
    }
}
