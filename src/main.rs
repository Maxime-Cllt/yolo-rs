use yolors::config::yolo_config::YoloConfig;
use yolors::session::yolo_session::YoloSession;

#[cfg(test)]
mod benches;

fn main() {
    // Determine the image path based on the build configuration
    let image_path: String = if cfg!(debug_assertions) {
        "assets/village_1759583271.png".into()
    } else {
        let args: Vec<String> = std::env::args().collect::<Vec<String>>();
        if args.len() < 2 {
            eprintln!("Usage cargo run --: {} <image_path>", args[0]);
            panic!("Not enough arguments");
        }
        args[1].clone()
    };

    // Use the embedded model bytes instead of a file path
    let mut yolo_model =
        YoloSession::new(YoloConfig::with_conf()).expect("Failed to create YOLO model");

    yolo_model
        .process_image(&image_path)
        .expect("Failed to process image");
}
