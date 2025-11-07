use serde::Deserialize;

/// This file is part of a Clash of Clans related project.
/// This file is part of a Clash of Clans related project.
#[derive(Deserialize)]
pub struct Class {
    pub id: u8,
    pub name: String,
    pub color: Option<String>,
}

impl Class {
    /// Returns the string representation of the `Class`.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_class_as_str() {
        let class = Class {
            id: 1,
            name: "test_class".to_string(),
            color: None,
        };
        assert_eq!(class.as_str(), "test_class");
    }

    #[test]
    fn test_class_deserialization() {
        const JSON_DATA: &str = r#"
        {
            "id": 2,
            "name": "example_class",
            "color": "\#FF5733"
        }
        "#;

        let class: Class = serde_json::from_str(JSON_DATA).unwrap();
        assert_eq!(class.id, 2);
        assert_eq!(class.name, "example_class");
        assert_eq!(class.color.unwrap(), "#FF5733");
    }
}
