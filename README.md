<p align="center">
<img height=80 src="icons/icon.svg" alt="Logo"/>
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

Download the latest DMG for your Mac architecture:

- **Apple Silicon**: [system-stats-aarch64.dmg](https://github.com/larskemper/macos-menu-bar-stats/releases/latest)
- **Intel**: [system-stats-x86_64.dmg](https://github.com/larskemper/macos-menu-bar-stats/releases/latest)

## Features

- 🔋 **Battery monitoring** - Real-time battery percentage and charging state
- 🧠 **CPU usage tracking** - Monitor overall CPU utilization across all cores
- 💾 **Memory usage tracking** - View used/total memory and utilization percentage

## Repository structure

This repository's contents are divided across the following primary sections:

- `/src` contains all Rust source code for the application
- `/icons` contains all application icons for macOS
- `/capabilities` contains Tauri security capabilities configuration
- `/gen` contains generated schema files from Tauri
- `/target` contains compiled binaries and build artifacts

## Tech Stack

**Core Framework:**
- [Tauri 2.0](https://tauri.app/) - Native application framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
## Development & Build

### Prerequisites

Ensure you have the following installed:
- Rust (latest stable)
- Xcode Command Line Tools (macOS)

### Development

Run the application in development mode with hot-reload:

```bash
cargo tauri dev
```

### Build

Build the application for production:

```bash
cargo tauri build
```

The built application bundle will be available in:
```
target/release/bundle/macos/
```

### Check & Test

Check code for errors without building:

```bash
cargo check
```

Run Clippy for linting:

```bash
cargo clippy
```

Format code:

```bash
cargo fmt
```