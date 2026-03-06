# Plan 016 — Auto-Update Checking & Install Update Button

## Context

The daemon already checks GitHub releases every 24h and caches `latest_version` in config, but this is invisible to users — it only logs to the daemon log file. The macOS app has no update notification UI. Users have no way to know a new version exists or install it without manually checking GitHub.

This plan adds:
1. CLI commands for the macOS app to query cached update status and trigger installation
2. macOS app UI with an auto-checking update banner and one-click install + restart

## Architecture

```
Daemon (background)          CLI (on-demand)              macOS App (UI)
─────────────────           ───────────────              ─────────────
checks GitHub every 24h     `update status` reads        calls CLI via process
writes latest_version       config → outputs JSON        shows update banner
to config.toml              `update install` downloads   "Install Update" button
                            DMG, mounts, copies .app     "Restart" after install
```

The app never hits GitHub directly — it reads the daemon's cached result via `slicky update status` (local-only, no network). When the user clicks "Install Update", it calls `slicky update install` which downloads the DMG, mounts it, replaces `/Applications/OpenSlicky.app`, and restarts the daemon.

## Files Modified

| File | Change |
|------|--------|
| `crates/slicky-cli/Cargo.toml` | Added `serde` dependency |
| `crates/slicky-cli/src/main.rs` | Added `Status`, `Install` to `UpdateAction` + dispatch |
| `crates/slicky-cli/src/update.rs` | Added `status()`, `install()`, helpers, JSON structs, tests |
| `macos/OpenSlicky/SlickyCLI.swift` | Added `updateStatus()`, `installUpdate()`, `installUpdateAdmin()` |
| `macos/OpenSlicky/OpenSlickyApp.swift` | Added Codable structs, ViewModel update state/methods, `UpdateBannerView`, integrated into both layouts |

## Key Design Decisions

- **No new API calls from the app** — relies on daemon's cached check (GitHub rate-limit safe)
- **Admin fallback** — tries normal file ops first, escalates via `osascript` admin prompt only if needed
- **Symlinks preserved** — replacing `.app` bundle doesn't break `/usr/local/bin/slicky` symlinks
- **Daemon auto-restart** — `launchctl stop` + LaunchAgent `KeepAlive = true` picks up new binary
- **DMG integrity verified** — hdiutil checksums enabled (no `-noverify` flag)
- **Mount point validated** — must start with `/Volumes/` to prevent path traversal
- **Streaming download** — DMG streamed directly to disk, not buffered in memory
- **Consistent JSON error reporting** — all error paths return JSON, no raw bail! errors
