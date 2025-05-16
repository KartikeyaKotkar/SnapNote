# 📸 SnapNote

**SnapNote** is a lightweight, cross-platform hotkey daemon written in **Rust** that allows you to instantly:

- 📷 Take screenshots  
- 📋 Save clipboard code snippets  
- 📝 Capture quick text notes  

All files are neatly organized in date-based directories for later review.

---

## ✅ Features

- ⌨️ Global hotkey support
- 🖼️ Screenshot capture (macOS: `screencapture`, Linux: `scrot`)
- 📎 Clipboard-based code snippet saving
- 🗒️ Placeholder note capture (can be extended to custom text input)
- 📂 Auto-organized into `~/SnapNote/YYYY/MM-DD/`

---

## ⚙️ Installation

### Prerequisites

- 🦀 [Rust](https://rustup.rs)
- 📷 Screenshot tool:
  - macOS: `screencapture` (pre-installed)
  - Linux: `scrot` → Install via:  
    ```bash
    sudo apt install scrot
    ```

### Clone & Build

```bash
git clone https://github.com/your-username/snapnote.git
cd snapnote
cargo build --release
