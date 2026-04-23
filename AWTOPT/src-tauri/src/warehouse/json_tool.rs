use serde::{Deserialize, Serialize};
use serde_json;

pub struct SerdeJsonTool;

impl SerdeJsonTool {
    pub fn new() -> Self {
        SerdeJsonTool
    }

    pub fn to_json<T: Serialize>(&self, value: &T) -> Result<String, String> {
        serde_json::to_string(value).map_err(|e| format!("Serialization error: {}", e))
    }

    pub fn from_json<T: for<'de> Deserialize<'de>>(&self, json: &str) -> Result<T, String> {
        serde_json::from_str(json).map_err(|e| format!("Deserialization error: {}", e))
    }
}

impl Default for SerdeJsonTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_serialize_deserialize() {
        let tool = SerdeJsonTool::new();
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        let json = tool.to_json(&data).unwrap();
        let restored: TestData = tool.from_json(&json).unwrap();

        assert_eq!(data, restored);
    }
}
