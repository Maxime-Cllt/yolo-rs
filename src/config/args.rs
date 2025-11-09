use crate::config::mode::Mode;

/// Parses command-line arguments to determine the mode of operation
#[inline]
pub fn parse_args(args: &[String]) -> Mode {
    if let Some(pos) = args.iter().position(|a| a == "--folder") {
        if let Some(folder) = args.get(pos + 1) {
            Mode::Folder(folder.clone())
        } else {
            eprintln!("Error: --folder flag requires a folder path");
            std::process::exit(1);
        }
    } else {
        Mode::Default(args.get(1).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_default() {
        let args = vec!["program".to_string(), "image.png".to_string()];
        match parse_args(&args) {
            Mode::Default(path) => assert_eq!(path, "image.png"),
            _ => panic!("Expected Default mode"),
        }
    }

    #[test]
    fn test_parse_args_folder() {
        let args = vec![
            "program".to_string(),
            "--folder".to_string(),
            "images/".to_string(),
        ];
        match parse_args(&args) {
            Mode::Folder(path) => assert_eq!(path, "images/"),
            _ => panic!("Expected Folder mode"),
        }
    }
}
