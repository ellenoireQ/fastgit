<div align="center">

# fastgit

![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Version](https://img.shields.io/badge/version-0.1.0-green)

**Just like a rocket**

</div>

---

## Interface
![interface_preview](docs/video/fastgit.gif)

## Table of Contents

- [Philosophy](#philosophy)
- [Features](#features)
- [Installation](#installation)
  - [Download pre-built binary](#download-pre-built-binary)
  - [Build from source](#build-from-source)
  - [Add to PATH](#add-to-path)
  - [Run](#run)
- [Interface](#interface)
- [Keybindings](#keybindings)
  - [Navigation](#navigation)
  - [Git Actions](#git-actions)
  - [Branch & Remote](#branch--remote)
  - [Input Fields](#input-fields)
  - [File Status Icons](#file-status-icons)
- [Built With](#built-with)
- [License](#license)

---

## Philosophy

Kill friction. Boost productivity.

---

## Features

- **File tree**: live view of your working tree with git status icons
- **Commit graph**: scrollable history with per-commit diff
- **Staging**: stage and unstage files with a single keypress
- **Commit**: write summary + description in a split dialog
- **Push**: push to any configured remote, no extra prompts
- **Branch management**: create, switch, and checkout branches inline
- **Remote management**: add and delete remotes without leaving the UI
- **Auto-refresh**: repo state syncs every 3 seconds automatically
- **Auto-pull**: background pull runs every 60 seconds

---

## Installation

**Supported platforms:**

![Linux](https://img.shields.io/badge/Linux-x86__64-blue?logo=linux)
![Windows](https://img.shields.io/badge/Windows-x86__64-blue?logo=windows)

### Download pre-built binary

Grab the latest binary from the [Releases](https://github.com/ellenoireQ/fastgit/releases) page.

| Platform | File |
|----------|------|
| Linux | `fastgit-vX.X.X-x86_64-unknown-linux-gnu.tar.gz` |
| Windows | `fastgit-vX.X.X-x86_64-pc-windows-msvc.zip` |

**Linux:**
```bash
tar -xzf fastgit-vX.X.X-x86_64-unknown-linux-gnu.tar.gz
chmod +x fastgit
mv fastgit ~/.local/bin/
```

**Windows:**

Extract the `.zip` and move `fastgit.exe` to a folder in your `PATH` (e.g. `C:\Users\you\bin\`).

### Build from source

Requires Rust toolchain — [install via rustup](https://rustup.rs).

```bash
git clone https://github.com/ellenoireQ/fastgit
cd fastgit
cargo build --release
```

Binary will be at `target/release/fastgit` (or `fastgit.exe` on Windows). Move it to a directory in your `PATH` (see [Add to PATH](#add-to-path) below).

### Add to PATH

If `~/.local/bin` is not yet in your `$PATH`, add it for your shell:

**bash** (`~/.bashrc` or `~/.bash_profile`):
```bash
export PATH="$HOME/.local/bin:$PATH"
```

**zsh** (`~/.zshrc`):
```zsh
export PATH="$HOME/.local/bin:$PATH"
```

**fish** (`~/.config/fish/config.fish`):
```fish
fish_add_path $HOME/.local/bin
```

Reload your shell after editing:
```bash
source ~/.bashrc   # or ~/.zshrc, or restart the terminal
```

**Windows:**

1. Open **System Properties** → **Advanced** → **Environment Variables**
2. Under **User variables**, select `Path` and click **Edit**
3. Click **New** and add the folder where you placed `fastgit.exe` (e.g. `C:\Users\you\bin`)
4. Click **OK** and restart your terminal

### Run

```bash
cd /your/repo
fastgit
```

---

## Keybindings

### Navigation

| Key | Action |
|-----|--------|
| `Tab` | Switch panel |
| `Up` / `Down` | Navigate list |
| `Left` / `Right` | Switch branch tab (Local / Remote) |
| `Enter` | Select file / checkout branch |
| `Esc` | Deselect / close dialog |
| `?` | Toggle help |
| `q` | Quit |

### Git Actions

| Key | Action |
|-----|--------|
| `Space` | Stage / unstage file |
| `c` | Commit staged changes |
| `P` | Push to remote |
| `s` | Rescan git status |

### Branch & Remote

| Key | Action |
|-----|--------|
| `n` | New branch (Local tab) |
| `Enter` | Checkout selected branch |
| `a` | Add remote (Remote tab) |
| `d` | Delete remote (Remote tab) |
| `Enter` | Set selected remote for push (Remote tab) |

### Input Fields

| Key | Action |
|-----|--------|
| `Left` / `Right` | Move cursor |
| `Home` / `End` | Jump to start / end |
| `Backspace` | Delete char before cursor |
| `Delete` | Delete char at cursor |
| `Tab` | Switch between Summary / Description |

### File Status Icons

| Icon | Meaning |
|------|---------|
| `S` | Staged |
| `M` | Modified |
| `N` | New (untracked) |
| `D` | Deleted |
| `R` | Renamed |
| `T` | Type changed |

---

## Built With

- [ratatui](https://github.com/ratatui/ratatui) — terminal UI framework
- [git2](https://github.com/rust-lang/git2-rs) — libgit2 bindings for Rust
- [crossterm](https://github.com/crossterm-rs/crossterm) — cross-platform terminal input
- [tokio](https://tokio.rs) — async runtime for background push/pull

---

## License

MIT — see [LICENSE](LICENSE).
