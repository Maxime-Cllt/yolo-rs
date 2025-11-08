<div align="center">
    <h1>🦀 YOLO-rs</h1>
    <p><em>A blazing-fast Rust runtime for YOLOv8 and YOLOv10 object detection using ONNX Runtime</em></p>
</div>

<div align="center">
  <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/ONNX_Runtime-005CED?style=for-the-badge&logo=onnx&logoColor=white" alt="ONNX Runtime" />
  <img src="https://img.shields.io/badge/Version-0.1.0-6c63ff?style=for-the-badge" alt="Version" />
  <img src="https://img.shields.io/badge/License-GPL--3.0-3C8DAD?style=for-the-badge&logo=open-source-initiative&logoColor=white" alt="License" />
</div>

<br/>

## 📖 Overview

**YOLO-rs** is a high-performance Rust runtime designed to execute YOLO models for object detection tasks. Built with speed and reliability in mind, it leverages ONNX Runtime to provide efficient inference on images, making it ideal for real-time computer vision applications, game analytics, and automated image processing pipelines.

### 🎯 Why YOLO-rs?

- **🚀 Performance-First**: Written in Rust for maximum speed and memory safety
- **🔧 Zero-Dependency Runtime**: Standalone executable with minimal dependencies
- **📦 Production-Ready**: Optimized build profiles for development, testing, and production
- **🎨 Visual Output**: Built-in image annotation with customizable bounding boxes
- **🔄 Flexible I/O**: Support for JSON and YOLO format outputs
- **⚡ ONNX Powered**: Leverages ONNX Runtime for cross-platform ML inference

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| **🤖 YOLOv8 Support** | Full support for YOLOv8 models in ONNX format |
| **🖼️ Image Processing** | Handles multiple image formats (PNG, JPG, etc.) |
| **🎯 Object Detection** | Accurate bounding box detection with confidence scores |
| **📊 Multiple Output Formats** | Export results as JSON or YOLO annotation format |
| **🎨 Visual Annotations** | Generate annotated images with colored bounding boxes |
| **⚙️ Configurable** | JSON-based configuration for models and classes |
| **🧪 Well-Tested** | Comprehensive unit tests and benchmarks included |

---

## 📋 Prerequisites

### Required

- **[Rust](https://www.rust-lang.org/tools/install)** (latest stable version)
- **ONNX Runtime** libraries (automatically handled by `ort` crate)

### Optional

- A YOLOv8 model exported to ONNX format
- `config.json` file with your model configuration

---

## 🚀 Installation

### Clone the Repository

```bash
git clone https://github.com/Maxime-Cllt/yolo-rs.git
cd yolo-rs
```

### Build the Project

```bash
# Development build
cargo build

# Optimized release build
cargo build --release
```

---

## 🎮 Usage

### Basic Usage

```bash
# Run inference on an image
cargo run --release -- path/to/image.png

# Or use the compiled binary
./target/release/yolo-rs path/to/image.png
```

### Configuration

Create a `config.json` file in your project root:

```json
{
  "model": {
    "path": "models/yolov8n.onnx",
    "input_size": 640,
    "confidence_threshold": 0.5,
    "iou_threshold": 0.45
  },
  "class": [
    {"id": 0, "name": "person"},
    {"id": 1, "name": "bicycle"},
    {"id": 2, "name": "car"}
  ]
}
```

### As a Library

Add `yolo-rs` to your `Cargo.toml`:

```toml
[dependencies]
yolors = { path = "path/to/yolo-rs" }
```

Use it in your code:

```rust
use yolors::{YoloConfig, YoloModel};

fn main() {
    // Load configuration
    let config = YoloConfig::with_conf();
    
    // Initialize model
    let model = YoloModel::new(&config).expect("Failed to load model");
    
    // Run inference
    let detections = model.detect("image.png").expect("Inference failed");
    
    println!("Found {} objects", detections.len());
}
```

---

## 📊 Output Formats

### JSON Format

```json
{
  "detections": [
    {
      "id": 1,
      "category_id": 0,
      "score": 0.9588687419891357,
      "x1": 427.95941162109375,
      "y1": 278.3505859375,
      "x2": 453.7296142578125,
      "y2": 303.136962890625,
      "width": 25.77020263671875,
      "height": 24.786376953125
    }
  ],
  "images": [
    {
      "file_name": "image.png",
      "width": 640,
      "height": 640
    }
  ]
}
```

### YOLO Format

```
0 0.668 0.454 0.040 0.039 0.9588
```

Format: `class_id center_x center_y width height confidence`

---

## 🧪 Testing & Benchmarking

### Run Unit Tests

```bash
cargo test
```

### Run Benchmarks

```bash
cargo bench
```

Benchmarks use the [Criterion](https://github.com/bheisler/criterion.rs) framework for accurate performance measurements.

---

## 🏗️ Project Structure

```
yolo-rs/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # CLI application
│   ├── model/              # YOLO model implementation
│   ├── config/             # Configuration handling
│   ├── utils/              # Utility functions
│   └── benches/            # Benchmark suite
├── models/                 # ONNX model files
├── tests/                  # Integration tests
├── config.json             # Configuration file
├── Cargo.toml              # Project manifest
└── README.md
```

---

## ⚙️ Performance

YOLO-rs is optimized for performance with:

- **LTO (Link Time Optimization)**: Enabled in release builds
- **Code Generation Units**: Optimized for maximum performance
- **Profile-Guided Optimization**: Multiple build profiles for different use cases
- **Memory Efficiency**: Zero-copy operations where possible

### Build Profiles

| Profile | Use Case | Optimization |
|---------|----------|--------------|
| `dev` | Development | Fast compilation, debugging |
| `test` | Unit testing | Balanced speed and compile time |
| `release` | Production | Maximum performance |
| `bench` | Benchmarking | Performance testing |

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Commit** your changes
   ```bash
   git commit -m 'Add amazing feature'
   ```
4. **Push** to your branch
   ```bash
   git push origin feature/amazing-feature
   ```
5. **Open** a Pull Request

### Development Guidelines

- Write tests for new features
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Update documentation as needed

---

## 📝 License

This project is licensed under the **GPL-3.0 License** - see the [LICENSE](LICENSE) file for details.

---