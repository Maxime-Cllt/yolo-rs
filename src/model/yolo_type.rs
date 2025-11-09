use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display};

/// Enum representing different types of YOLO models.
#[derive(PartialEq, Eq, Clone)]
pub enum YoloType {
    YoloV8,
    YoloV10,
}

impl YoloType {
    /// Returns the string representation of the `YoloType` variant.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::YoloV8 => "YoloV8",
            Self::YoloV10 => "YoloV10",
        }
    }
}

impl TryFrom<&str> for YoloType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "yolov8" => Ok(Self::YoloV8),
            "yolov10" => Ok(Self::YoloV10),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for YoloType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(Self::YoloV8),
            10 => Ok(Self::YoloV10),
            _ => Err(()),
        }
    }
}

impl Display for YoloType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Custom deserializer implementation
impl<'de> Deserialize<'de> for YoloType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        YoloType::try_from(value).map_err(|_| serde::de::Error::custom("Invalid YoloType value"))
    }
}

impl Debug for YoloType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yolo_type_as_str() {
        assert_eq!(YoloType::YoloV8.as_str(), "YoloV8");
        assert_eq!(YoloType::YoloV10.as_str(), "YoloV10");
    }

    #[test]
    fn test_yolo_type_try_from() {
        assert_eq!(YoloType::try_from("yolov8").unwrap(), YoloType::YoloV8);
        assert_eq!(YoloType::try_from("YoloV8").unwrap(), YoloType::YoloV8);
        assert_eq!(YoloType::try_from("YOLOV8").unwrap(), YoloType::YoloV8);
        assert_eq!(YoloType::try_from("yolov10").unwrap(), YoloType::YoloV10);
        assert_eq!(YoloType::try_from("YoloV10").unwrap(), YoloType::YoloV10);
        assert_eq!(YoloType::try_from("YOLOV10").unwrap(), YoloType::YoloV10);
        assert!(YoloType::try_from("unknown").is_err());
    }
}
