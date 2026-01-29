# AI Notification Filter

A Rust-based tool that uses a 0.6B language model (e.g., Qwen3-0.6B) to intelligently filter macOS notifications based on the currently focused application window.

## Features

- **AI-Powered Filtering**: Uses MLX/Candle framework to load and run inference on a quantized LLM
- **Context-Aware**: Analyzes the current application and window title to determine notification relevance
- **macOS Integration**: Leverages Accessibility API to monitor and interact with system notifications
- **Smart Decisions**: 
  - Work/learning contexts (e.g., Chrome with dev docs): Allows work-related notifications
  - Entertainment contexts (e.g., Chrome with videos/games): Allows all notifications
  - Filters out irrelevant or spam notifications

## Requirements

- macOS 12+ with Apple Silicon (M1/M2/M3) for Metal acceleration
- Rust 1.70+
- Accessibility permissions (enable in System Settings > Privacy & Security > Accessibility)

## Installation

1. Clone or download this project
2. Install dependencies:
   ```bash
   cargo build --release
   ```

## Usage

1. Enable Accessibility permissions for the built binary
2. Run the tool:
   ```bash
   cargo run --release
   ```

The tool will run in the background, monitoring notifications and filtering them based on your current activity.

## Configuration

- Model: Currently configured for Qwen2.5-0.5B GGUF (adjust in `src/main.rs` for Qwen3-0.6B when available)
- Device: Automatically uses Metal GPU on Apple Silicon, falls back to CPU

## Troubleshooting

- **Accessibility Permissions**: If notifications aren't being monitored, ensure the app has Accessibility access
- **Model Loading**: Ensure the model file is correctly downloaded and path is set
- **Performance**: On first run, model loading may take time; subsequent runs are faster

## Architecture

Based on the reference document, this implementation uses:
- `candle-core` & `candle-nn` for LLM inference
- `active-win` for current window detection
- `accessibility-sys` & `objc2` for macOS API integration
- Tokio for async runtime

## Development

The core components:
- `get_context()`: Retrieves current focused window
- `run_inference()`: Runs LLM to decide notification display
- `NotificationWatcher`: Monitors system notifications via Accessibility API

## License

MIT License