use crate::config::class::Class;
use crate::config::model_config::ModelConfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct YoloConfig {
    pub model: ModelConfig,
    #[serde(rename = "class")]
    pub classes: Vec<Class>,
}

impl YoloConfig {
    /// Returns the number of classes in the configuration.
    #[inline]
    #[must_use]
    pub const fn num_classes(&self) -> usize {
        self.classes.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::class::Class;
    use crate::config::model_config::ModelConfig;
    use crate::config::yolo_config::YoloConfig;
    use crate::model::yolo_type::YoloType;

    #[test]
    fn test_num_classes() {
        let config = YoloConfig {
            model: ModelConfig {
                path: "best.onnx".into(),
                architecture: YoloType::YoloV8,
            },
            classes: vec![Class {
                id: 0,
                name: "person".to_string(),
                color: None,
            }],
        };
        assert_eq!(config.num_classes(), 1);
    }
}
