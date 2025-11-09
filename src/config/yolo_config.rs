use crate::config::class::Class;
use crate::config::model_config::ModelConfig;
use serde::Deserialize;
use std::fs;

// Load YOLO configuration lazily at runtime
pub static YOLO_CONFIG: std::sync::LazyLock<YoloConfig> = std::sync::LazyLock::new(|| YoloConfig::with_conf());

/// Configuration for YOLO model and classes.
#[derive(Deserialize, Clone)]
#[must_use]
#[non_exhaustive]
pub struct YoloConfig {
    pub model: ModelConfig,
    #[serde(rename = "class")]
    pub classes: Vec<Class>,
}

impl YoloConfig {
    /// Loads the YOLO configuration from a JSON file.
    #[inline]
    pub fn with_conf() -> Self {
        let data = fs::read_to_string("config.json").expect("Unable to read file");
        let yolo_conf: Self =
            serde_json::from_str(&data).expect("JSON was not well-formatted");
        yolo_conf
    }

    /// Returns the number of classes in the configuration.
    #[inline]
    #[must_use]
    pub const fn num_classes(&self) -> usize {
        self.classes.len()
    }

    /// Returns a vector of RGBA colors for each class.
    #[inline]
    #[must_use]
    pub fn colors(&self) -> Vec<(u8, u8, u8, u8)> {
        self.classes
            .iter()
            .map(|class| {
                if let Some(color_hex) = &class.color {
                    let hex = color_hex.trim_start_matches('#');
                    if hex.len() == 6 {
                        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                        let a = 255;
                        (r, g, b, a)
                    } else {
                        // Default to white if the format is incorrect
                        (255, 255, 255, 255)
                    }
                } else {
                    // Random color generation as fallback
                    let r = rand::random::<u8>();
                    let g = rand::random::<u8>();
                    let b = rand::random::<u8>();
                    let a = 255;
                    (r, g, b, a)
                }
            })
            .collect()
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
