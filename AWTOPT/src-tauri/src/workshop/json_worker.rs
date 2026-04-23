use crate::contracts::JsonTrait;
use crate::warehouse::SerdeJsonTool;

pub struct JsonWorker {
    tool: SerdeJsonTool,
}

impl JsonWorker {
    pub fn new() -> Self {
        Self {
            tool: SerdeJsonTool::new(),
        }
    }
}

impl Default for JsonWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonTrait for JsonWorker {
    fn to_json<T: serde::Serialize>(&self, value: &T) -> Result<String, String> {
        self.tool.to_json(value)
    }

    fn from_json<T: for<'de> serde::Deserialize<'de>>(&self, json: &str) -> Result<T, String> {
        self.tool.from_json(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{JsonTrait, SettingsUpdateDto};

    #[test]
    fn test_json_worker() {
        let worker = JsonWorker::new();

        let request = SettingsUpdateDto {
            work_duration: Some(30),
            short_break_duration: Some(10),
            long_break_duration: None,
            auto_start_breaks: None,
            sequence_list: None,
        };

        // Serialize
        let json = worker.to_json(&request).unwrap();
        assert!(json.contains("30"));

        // Deserialize
        let restored: SettingsUpdateDto = worker.from_json(&json).unwrap();
        assert_eq!(restored.work_duration, Some(30));
        assert_eq!(restored.short_break_duration, Some(10));
    }
}
