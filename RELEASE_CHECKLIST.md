# Release Checklist

Quick reference for releasing a new version of System Stats.

## Release Steps

### 1. Bump Version

```bash
./scripts/bump-version.sh 0.2.0
```

This updates:
- `Cargo.toml`
- `tauri.conf.json`
- `Cargo.lock`

### 2. Commit Changes

```bash
git add Cargo.toml tauri.conf.json Cargo.lock
git commit -m "Bump version to 0.2.0"
```

### 3. Run Tests & Lints

```bash
cargo test
cargo clippy
cargo fmt --check
```

### 4. Build Release

```bash
./scripts/build-release.sh
```

Creates:
- `release/system-stats-aarch64.dmg` (Apple Silicon)
- `release/system-stats-x86_64.dmg` (Intel)

### 5. Get Checksums

```bash
./scripts/get-checksums.sh
```

Copy the SHA256 values for Homebrew cask update.

### 6. Create Tag & Push

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main --tags
```

### 7. Create GitHub Release

1. Go to: https://github.com/larskemper/macos-menu-bar-stats/releases/new
2. Select tag: `v0.2.0`
3. Upload DMG files from `release/` directory
4. Add release notes
5. Publish release

### 8. Update Homebrew Cask

If you have a `homebrew-cask` tap:

```bash
cd /tmp
git clone https://github.com/larskemper/homebrew-cask.git
cd homebrew-cask
```

Edit `Casks/system-stats.rb`:
```ruby
version "0.2.0"

sha256 arm:   "new_aarch64_checksum",
       intel: "new_x86_64_checksum"
```

```bash
git add Casks/system-stats.rb
git commit -m "Update system-stats to v0.2.0"
git push origin main
```

### 9. Test Installation

```bash
brew upgrade --cask system-stats
```

## Quick Commands

```bash
# Full release workflow
./scripts/bump-version.sh 0.2.0
git add Cargo.toml tauri.conf.json Cargo.lock
git commit -m "Bump version to 0.2.0"
cargo test && cargo clippy
./scripts/build-release.sh
./scripts/get-checksums.sh
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main --tags
```
