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

> **Note:** The distributed binaries are shipped with the macOS quarantine flag removed 

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