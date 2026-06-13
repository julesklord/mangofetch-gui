<table border="0">
  <tr>
    <td width="200" align="center" valign="top">
      <img src="docs/assets/logo.svg" width="180" alt="MangoFetch logo">
    </td>
    <td valign="top">
      <h1>mangofetch-gui</h1>
      <p><strong>Hardware-Accelerated, Premium Rust GUI.</strong><br/>
      <em>The graphical desktop client frontend of the mangoSuite.</em></p>
      <p>
        <a href="https://crates.io/crates/mangofetch-gui"><img src="https://img.shields.io/crates/v/mangofetch-gui?style=plastic&color=orange" alt="Crates.io"></a>
        <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue?style=plastic" alt="License GPL-3.0"></a>
        <img src="https://img.shields.io/badge/Built%20With-Rust-red?style=plastic&logo=rust" alt="Built with Rust">
        <img src="https://img.shields.io/badge/GUI-egui-lightblue?style=plastic" alt="egui">
        <img src="https://img.shields.io/badge/Aesthetics-MonolithUI-black?style=plastic" alt="MonolithUI">
      </p>
    </td>
  </tr>
</table>

---

<!--toc:start-->
- [Overview](#overview)
- [The mangoSuite](#the-mangosuite)
- [Key Features](#key-features)
- [GUI Installation](#gui-installation)
  - [Via Cargo (Recommended)](#via-cargo-recommended)
  - [From Source](#from-source)
- [How to Run](#how-to-run)
- [License](#license)
<!--toc:end-->

## Overview

`mangofetch-gui` is a premium, hardware-accelerated desktop application designed for managing downloads. Powered by `egui` and `eframe`, it implements the **MonolithUI** industrial design language, featuring sleek dark mode options, smooth physics-based progress meters, and dynamic telemetry widgets.

Under the hood, it harnesses the headless asynchronous download power of **`mangofetch-core`** to fetch media and torrent streams.

## The mangoSuite

`mangofetch-gui` is the graphical companion client in the `mangoSuite` ecosystem:
* **[mangofetch](https://github.com/julesklord/mangofetch)**: Core engine SDK and interactive Ratatui TUI.
* **[mangofetch-cli](https://github.com/julesklord/mangofetch-cli)**: Scriptable, non-interactive CLI frontend.
* **[mangofetch-gui](https://github.com/julesklord/mangofetch-gui)** (This repo): Premium desktop application.

## Key Features

* **MonolithUI Aesthetic:** Sleek dark industrial HUD styling, customizable HSL-based palettes, and modern typography.
* **Granular Controls:** Choose audio and video qualities, format extensions, and directories visually.
* **Interactive Telemetry:** Live download rate graphs, estimated completion times, and speed gauges.
* **Recovery Panel:** Browse download history, retry failed links, or pause and resume active downloads with a click.
* **Automatic Provisioning:** Fully automated background checking and downloading for dependency tools like `yt-dlp` and `ffmpeg`.

---

## GUI Installation

### Via Cargo (Recommended)

Install the GUI client directly:

```zsh
cargo install mangofetch-gui
```

### From Source

Ensure you have your system's graphics drivers and window development packages installed (e.g., `libx11-dev` and `libasound2-dev` on Linux).

```zsh
git clone https://github.com/julesklord/mangofetch-gui.git
cd mangofetch-gui
cargo build --release
```

---

## How to Run

Launch the application using:

```zsh
mangofetch-gui
```

## Local Development

Since this repository resolves `mangofetch-core` directly from crates.io for distribution, you can configure your local workspace to use a path override for development. Add the following to the end of the root `Cargo.toml` in your local development workspace to point to your local copy of `mangofetch`:

```toml
[patch.crates-io]
mangofetch-core = { path = "../mangofetch/mangofetch-core" }
mangofetch-plugin-sdk = { path = "../mangofetch/mangofetch-plugin-sdk" }
```

---

## License

<p align="center">
  Built by <a href="https://github.com/julesklord">Jules</a> and Claude.<br>
  Released under the GPL-3.0 License.
</p>
