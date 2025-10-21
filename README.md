# Mac Menu Bar Stats

A pure Rust/Tauri macOS menu bar application that displays system statistics (battery, CPU, and memory usage) in the menu bar.

## Features

- 🔋 Battery percentage and charging status
- 🧠 CPU usage monitoring
- 💾 Memory usage monitoring
- Click menu items to copy values to clipboard
- Updates every 2 seconds

## Development

```bash
pnpm install
pnpm dev
```

## Build

```bash
pnpm build
```

The built app will be in `src-tauri/target/release/bundle/`.

## Tech Stack

- Rust + Tauri (no frontend/JavaScript)
- Native macOS menu bar integration
- System monitoring via `sysinfo` and `battery` crates
