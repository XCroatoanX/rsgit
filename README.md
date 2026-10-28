<div align="center">

  <img src="https://github.com/XCroatoanX/rsgit/blob/master/assets/logo.png" alt="rsgit Logo" width="200" />

# rsgit

  **A fast, keyboard-driven Terminal User Interface (TUI) Git client written in Rust.**

  [![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

  [Features](#features) • [Requirements](#requirements) • [Quick Start](#quick-start) • [Key Bindings](#key-bindings) • [Development](#development)

</div>

---

## ⚡ Features

* 🌿 **Ref Management:** Browse local branches, remote branches, and tags.
* 🔀 **Branch Actions:** Checkout, create, rename, and delete refs effortlessly.
* 🔄 **Merge & Rebase:** Execute merges and rebases with selectable strategies.
* 🚀 **Syncing:** Pull and push with real-time inline progress.
* 📜 **History & Diffs:** Inspect commit history and view detailed commit diffs.
* ⌨️ **Keyboard-Driven:** Navigate seamlessly with intuitive shortcuts and contextual help.

---

## 📋 Requirements

* [Git](https://git-scm.com/) installed on your system.
* **Rust 1.85+** (uses the Rust 2024 edition).

---

## 🚀 Quick Start

### Build and Run

1. **Clone the repository:**

   ```sh
   git clone https://github.com/XCroatoanX/rsgit.git
   cd rsgit
   ```

2. **Run in the current directory:**

   ```sh
   cargo run --release
   ```

3. **Open a specific repository:**

   ```sh
   cargo run --release -- /path/to/repository
   ```

---

## ⌨️ Key Bindings

### Navigation & Panes

| Key | Action |
| :--- | :--- |
| <kbd>1</kbd> – <kbd>5</kbd> | Jump to specific pane |
| <kbd>Tab</kbd> / <kbd>h</kbd> | Move to next pane |
| <kbd>Shift</kbd> + <kbd>Tab</kbd> / <kbd>l</kbd> | Move to previous pane |
| <kbd>j</kbd> / <kbd>↓</kbd> | Move selection down |
| <kbd>k</kbd> / <kbd>↑</kbd> | Move selection up |
| <kbd>[</kbd> / <kbd>]</kbd> | Switch branch tabs |

### Actions & Operations

| Key | Action |
| :--- | :--- |
| <kbd>Space</kbd> | Checkout selected ref / stage or unstage file |
| <kbd>Enter</kbd> | Fold / unfold folder / confirm popup |
| <kbd>n</kbd> | Create branch or tag |
| <kbd>r</kbd> | Rename selected item |
| <kbd>d</kbd> | Delete selected item |
| <kbd>p</kbd> / <kbd>P</kbd> | Pull / push selected branch |
| <kbd>M</kbd> | Merge current branch into selected branch |
| <kbd>R</kbd> | Rebase current branch onto selected branch |

### System & Help

| Key | Action |
| :--- | :--- |
| <kbd>?</kbd> | Toggle help overlay |
| <kbd>a</kbd> | Toggle about overlay |
| <kbd>q</kbd> / <kbd>Esc</kbd> | Close popup / quit application |

---

## 🛠️ Development

Ensure code quality before submitting a pull request or committing changes:

```sh
# Format check
cargo fmt --check

# Lint check
cargo clippy --all-targets -- -D warnings

# Run tests
cargo test
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
