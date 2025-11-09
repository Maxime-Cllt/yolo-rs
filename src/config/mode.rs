/// Module for parsing command-line arguments
pub enum Mode {
    Default(String),
    Folder(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_variants() {
        let default_mode = Mode::Default("image.png".to_string());
        let folder_mode = Mode::Folder("images/".to_string());

        match default_mode {
            Mode::Default(path) => assert_eq!(path, "image.png"),
            _ => panic!("Expected Default mode"),
        }

        match folder_mode {
            Mode::Folder(path) => assert_eq!(path, "images/"),
            _ => panic!("Expected Folder mode"),
        }
    }
}
