# 🥭 MangoFetch GUI Wiki

Welcome to the official documentation for **mangofetch-gui**, the hardware-accelerated desktop client frontend of the mangoSuite.

`mangofetch-gui` features a modern dark industrial aesthetic powered by `egui` and `eframe`. It implements the **MonolithUI** design system, featuring customizable HSL-based palettes, real-time download speed charts, estimated completion times (ETA), and a visual recovery panel. It uses the asynchronous core download engine (`mangofetch-core`) under the hood.

---

## 🧭 Navigation

### 🚀 Getting Started
*   **[Installation](Installation.md)**: How to install `mangofetch-gui` and compile it with the graphics libraries for your OS.
*   **[Quick Start Guide](Quick-Start.md)**: Master downloading files visually in 60 seconds.

### 🛠️ User Guides
*   **[Configuration](Configuration.md)**: Customizing download limits, theme colors, and paths visually.

### 🏗️ Technical Architecture
*   **[The Core Engine](Architecture.md)**: Understanding the download logic and process streams.
*   **[Platform Registry](Platforms.md)**: Platforms supported dynamically by the engine.

---

## 🍊 The mangoSuite
`mangofetch-gui` is the desktop client frontend of the suite:
*   **[mangofetch (TUI)](https://github.com/julesklord/mangofetch)**: Core engine SDK and Master Terminal User Interface.
*   **[mangofetch-cli](https://github.com/julesklord/mangofetch-cli)**: Scriptable, non-interactive CLI frontend.

---

## 🛡️ License
MangoFetch is licensed under the **GPL-3.0-or-later**.
