use serde::Deserialize;
use crate::model::yolo_type::YoloType;

#[derive(Deserialize)]
pub struct ModelConfig {
    pub path: String,
    pub architecture: YoloType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::yolo_type::YoloType;

    #[test]
    fn test_model_config_deserialization() {
        let json_data = r#"
        {
            "path": "model.onnx",
            "architecture": "YoloV8"
        }
        "#;

        let config: ModelConfig = serde_json::from_str(json_data).unwrap();
        assert_eq!(config.path, "model.onnx");
        assert_eq!(config.architecture, YoloType::YoloV8);
    }
}