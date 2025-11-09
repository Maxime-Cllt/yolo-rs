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

**YOLO-rs** is a high-performance Rust runtime designed to execute YOLO models for object detection tasks. Built with
speed and reliability in mind, it leverages ONNX Runtime to provide efficient inference on images, making it ideal for
real-time computer vision applications, game analytics, and automated image processing pipelines.

### 🎯 Why YOLO-rs?

- **🚀 Performance-First**: Written in Rust for maximum speed and memory safety
- **🔧 Zero-Dependency Runtime**: Standalone executable with minimal dependencies
- **📦 Production-Ready**: Optimized build profiles for development, testing, and production
- **🎨 Visual Output**: Built-in image annotation with customizable bounding boxes
- **🔄 Flexible I/O**: Support for JSON and YOLO format outputs
- **⚡ ONNX Powered**: Leverages ONNX Runtime for cross-platform ML inference

---

## ✨ Key Features

| Feature                        | Description                                            |
|--------------------------------|--------------------------------------------------------|
| **🤖 YOLOv8 Support**          | Full support for YOLOv8 models in ONNX format          |
| **🖼️ Image Processing**       | Handles multiple image formats (PNG, JPG, etc.)        |
| **🎯 Object Detection**        | Accurate bounding box detection with confidence scores |
| **📊 Multiple Output Formats** | Export results as JSON or YOLO annotation format       |
| **🎨 Visual Annotations**      | Generate annotated images with colored bounding boxes  |
| **⚙️ Configurable**            | JSON-based configuration for models and classes        |
| **🧪 Well-Tested**             | Comprehensive unit tests and benchmarks included       |

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
    "path": "model.onnx",
    "architecture": 8
  },
  "class": [
    {
      "id": 0,
      "name": "Cat",
      "color": "#FF0000"
    },
    {
      "id": 1,
      "name": "Dog",
      "color": "#00FF00"
    }
  ]
}
```

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

---

## 🧪 Code quality

### Unit Tests available

The `tests` directory is tested using the command :

```bash
cargo test
```

### Benchmarking available

Code is benchmarked using the `criterion` crate. To run benchmarks, use:

```bash
cargo bench
```

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
6. 
---

## 📝 License

This project is licensed under the **GPL-3.0 License** - see the [LICENSE](LICENSE) file for details.

---