use yolors::config::args::parse_args;
use yolors::config::mode::Mode;
use yolors::config::yolo_config::YOLO_CONFIG;
use yolors::session::yolo_session::YoloSession;

#[cfg(test)]
mod benches;

fn main() {
    // Determine the image path based on the build configuration
    let mode = parse_args(std::env::args().collect::<Vec<String>>().as_slice());

    // Use the embedded model bytes instead of a file path
    let mut yolo_model =
        YoloSession::new(YOLO_CONFIG.clone()).expect("Failed to create YOLO model");

    match mode {
        Mode::Default(image_path) => {
            if image_path.is_empty() {
                eprintln!("Error: No image path provided");
                std::process::exit(1);
            }
            yolo_model
                .process_image(&image_path)
                .expect("Failed to process image");
        }
        Mode::Folder(folder_path) => {
            let paths = std::fs::read_dir(&folder_path)
                .expect("Failed to read folder")
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    if let Some(ext) = path.extension() {
                        ext == "jpg" || ext == "png" || ext == "jpeg"
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();

            for path in paths {
                yolo_model
                    .process_image(path.to_str().unwrap())
                    .expect("Failed to process image");
            }
        }
    }
}
