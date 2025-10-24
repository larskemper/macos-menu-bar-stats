<p align="center">
<img height=80 src=".github/assets/logo.png" alt="Logo"/>
</p>

<p align="center">
  <strong>mac-menu-bar-stats</strong>
</p>

<p align="center">
Simple macOS menu bar app for system monitoring
</p>

<p align="center">
  <img src="https://github.com/larskemper/macos-menu-bar-stats/actions/workflows/ci.yml/badge.svg" alt="CI" />
  <img src="https://github.com/larskemper/macos-menu-bar-stats/actions/workflows/security.yml/badge.svg" alt="Security Audit" />
</p>

---

## Overview

- [Installation](#installation)
- [Features](#features)
- [Repository structure](#repository-structure)
- [Tech Stack](#tech-stack)
- [Development & Build](#development--build)

## Installation

### Via Homebrew (Recommended)
```bash
brew install --cask larskemper/cask/system-stats
```

### Manual Installation

Download the DMG for your architecture:

- **Apple Silicon**: [system-stats-aarch64.dmg](https://github.com/larskemper/macos-menu-bar-stats/releases/latest)
- **Intel**: [system-stats-x86_64.dmg](https://github.com/larskemper/macos-menu-bar-stats/releases/latest)

### macOS Quarantine Flags

The distributed DMG files are built **without** macOS quarantine flags, so they should install without Gatekeeper warnings.

If you encounter a "cannot be opened because the developer cannot be verified" message, you can manually remove the quarantine flag:

```bash
# Remove quarantine from the app
xattr -dr com.apple.quarantine "/Applications/System Stats.app"

# Verify removal (should return "No such xattr" or no output)
xattr -l "/Applications/System Stats.app"
```

Then try opening the app again.

## Features

- Battery percentage and charging state
- CPU usage across all cores
- Memory usage and utilization percentage

## Repository Structure

- `/src` - Rust source code
- `/icons` - Application icons
- `/capabilities` - Tauri security capabilities
- `/gen` - Generated schema files
- `/target` - Build artifacts

## Tech Stack

- [Tauri 2.0](https://tauri.app/)
- [Rust](https://www.rust-lang.org/)

## Development & Build

### Prerequisites

Ensure you have the following installed:
- Rust (latest stable)
- Xcode Command Line Tools

### Development

Run in development mode:

```bash
cargo run tauri dev
```

### Build

Build the application:

```bash
cargo build
```

Output location:
```
target/release/bundle/macos/
```

### Check & Test

Check for errors:

```bash
cargo check
```

Run linter:

```bash
cargo clippy
```

Format code:

```bash
cargo fmt
```