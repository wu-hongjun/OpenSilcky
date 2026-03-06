# Plan 006 — macOS .app Bundle & Drag-to-Install DMG

## Context

The current DMG contains raw binaries (`statuslight`, `statuslightd`, FFI artifacts) in a flat folder. Users don't know what to do with them. We need a proper macOS .app bundle that users drag to `/Applications`, with a first-launch installer that sets up CLI symlinks and the LaunchAgent.

## App Bundle Structure

```
StatusLight.app/
  Contents/
    MacOS/
      StatusLight       (launcher shell script — main executable)
      statuslight      (CLI binary)
      statuslightd          (daemon binary)
    Info.plist
    PkgInfo
```

No icon for now (macOS shows default). No Rust code changes needed.

## Launcher Script (`Contents/MacOS/StatusLight`)

When the user double-clicks the app:

1. **Detect App Translocation** — if running from DMG/temp path, show "drag to Applications first" error and exit
2. **Check versioned marker** (`~/.config/statuslight/.installed-<version>`) — if present, show "already installed" dialog with OK + Uninstall buttons
3. **First-time install:**
   - `osascript` prompts for admin password with explanation
   - `ln -sf` symlinks `/usr/local/bin/statuslight` and `/usr/local/bin/statuslightd` → binaries inside the .app
   - Runs `statuslight startup enable` (installs LaunchAgent pointing to statuslightd inside the .app)
   - Writes marker file
   - Shows success dialog
4. **Uninstall** (button in "already installed" dialog):
   - Runs `statuslight startup disable`
   - Removes `/usr/local/bin` symlinks (admin prompt)
   - Removes marker files
   - Shows confirmation (preserves `config.toml`)

**Why symlinks work:** `startup.rs`'s `find_statuslightd()` calls `std::env::current_exe()` which resolves symlinks on macOS. So `/usr/local/bin/statuslight` resolves to `/Applications/StatusLight.app/Contents/MacOS/statuslight`, and `exe.with_file_name("statuslightd")` correctly finds the sibling binary.

## Info.plist

- `CFBundleIdentifier`: `com.statuslight.app` (distinct from `com.statuslight.daemon`)
- `CFBundleExecutable`: `StatusLight` (the launcher script)
- `LSUIElement`: `true` (no Dock icon — the app runs briefly and exits)
- `CFBundleVersion` / `CFBundleShortVersionString`: substituted from git tag at build time

## DMG Layout

Uses `create-dmg --app-drop-link` for the standard drag-to-install experience:
- App icon at (150, 150)
- /Applications alias at (350, 150)

## FFI Artifacts

Shipped separately as `StatusLight-FFI-<tag>.zip` attached to the GitHub Release. Not inside the .app.

## Files Created/Modified

| File | Action |
|------|--------|
| `macos/Info.plist.template` | **Created** — plist with `${VERSION}` placeholder |
| `scripts/build-app.sh` | **Created** — builds .app structure from release binaries |
| `.github/workflows/release.yml` | **Modified** — .app bundle DMG + separate FFI zip |
| `.gitignore` | **Modified** — added `*.dmg` |
| `docs/plans/006-app-bundle.md` | **Created** — this plan |

## Verification

1. `bash scripts/build-app.sh 0.1.0` creates `target/release/StatusLight.app/` with correct structure
2. DMG opens showing .app and /Applications alias side by side
3. Double-clicking .app from /Applications shows install dialog, creates symlinks, starts daemon
4. `which statuslight` returns `/usr/local/bin/statuslight`
5. `statuslight set green` works from terminal
6. Double-clicking .app again shows "already installed" dialog
7. "Uninstall" removes symlinks, stops daemon, removes LaunchAgent
8. Running .app directly from DMG (without dragging) shows translocation warning
