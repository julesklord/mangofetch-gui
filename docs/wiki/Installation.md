# 🚀 Installation Guide

`mangofetch-gui` is a desktop application powered by `egui` and `eframe`.

## 📋 System Requirements

*   **Operating Systems**: Windows 10/11, Linux, macOS.
*   **Graphics Driver**: A system with OpenGL or Vulkan support.
*   **Linux Dependencies**: On Linux, compiling requires development libraries for window management and audio:
    ```bash
    # Ubuntu/Debian
    sudo apt-get install -y libx11-dev libasound2-dev libxkbcommon-dev libwayland-dev
    ```

---

## 🛠️ Mandatory Dependencies

`mangofetch-gui` uses external engines for specific platform downloads. Ensure these are installed:

1.  **[FFmpeg](https://ffmpeg.org/)**: Essential for merging audio/video streams.
2.  **[yt-dlp](https://github.com/yt-dlp/yt-dlp)**: Engine for video extraction.

> [!TIP]
> The GUI automatically runs checkups and can download these tools dynamically in the background when first opened.

---

## 📦 Installing mangofetch-gui

### 1. Via Cargo (Recommended)
You can compile and install it directly from crates.io:
```bash
cargo install mangofetch-gui
```

### 2. Pre-built Binaries
Download the latest release ZIP/Tarball for your OS from the [Releases](https://github.com/julesklord/mangofetch-gui/releases) page and run the executable.

### 3. Building from Source
To build the client yourself:

```bash
# Clone the repository
git clone https://github.com/julesklord/mangofetch-gui.git
cd mangofetch-gui

# Build the GUI
cargo build --release

# The executable will be at:
# ./target/release/mangofetch-gui
```
