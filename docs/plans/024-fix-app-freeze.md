# Plan 024 — Fix App Launch Freeze

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix the system-wide macOS freeze triggered by StatusLight.app launch.

**Architecture:** The freeze has one root cause (HidApi::new() blocking on macOS IOKit) amplified by two compounding issues (SwiftUI process accumulation without timeouts, LaunchAgent crash-restart loop without throttle). We fix all three layers: Rust HID backend, Swift process management, and LaunchAgent configuration.

**Tech Stack:** Rust (hidapi crate), Swift/SwiftUI (Process/Foundation), macOS LaunchAgent plist

---

## Root Cause Summary

1. **PRIMARY:** `hidapi = "2"` uses the macOS IOKit backend which can block indefinitely in `HidApi::new()` when IOKit enumeration stalls (no CFRunLoop, misbehaving device, or concurrent access).
2. **AMPLIFIER 1:** SwiftUI ViewModel spawns 4 CLI processes on init. Every 5s, 2 more are spawned. `Task.cancel()` does NOT kill the underlying `Process`. No timeout on `Process.waitUntilExit()`. GCD thread pool exhausts in ~2.5 minutes.
3. **AMPLIFIER 2:** LaunchAgent has `KeepAlive=true` with no `ThrottleInterval`. Daemon crash → immediate restart → more IOKit saturation.

## Task 1: Fix hidapi backend configuration

**Files:**
- Modify: `Cargo.toml:23`

**Step 1: Change hidapi dependency to use macos-shared-device feature**

In `Cargo.toml`, replace line 23:

```toml
hidapi = "2"
```

with:

```toml
hidapi = { version = "2", features = ["macos-shared-device"] }
```

The `macos-shared-device` feature configures IOKit for non-exclusive device access and has been reported to fix enumeration hangs on macOS. If this alone doesn't fix the hang, we add an explicit timeout wrapper in Task 2.

**Step 2: Verify it compiles**

Run: `cargo build --workspace --release 2>&1 | tail -5`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "fix(core): enable hidapi macos-shared-device to prevent IOKit hang"
```

---

## Task 2: Add timeout wrapper for HidApi::new() in DeviceRegistry

**Files:**
- Modify: `crates/statuslight-core/src/registry.rs`

**Step 1: Add a timeout-guarded HidApi constructor**

Replace the three separate `hidapi::HidApi::new()` calls with a shared helper that enforces a timeout. This prevents indefinite hangs even if the IOKit fix in Task 1 is insufficient.

In `registry.rs`, add a helper function and refactor:

```rust
use std::time::Duration;

/// Construct a HidApi with a timeout to prevent IOKit hangs on macOS.
/// Returns None if initialization takes longer than the timeout.
fn new_hidapi_with_timeout(timeout: Duration) -> Option<hidapi::HidApi> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let result = hidapi::HidApi::new();
        let _ = tx.send(result);
    });
    match rx.recv_timeout(timeout) {
        Ok(Ok(api)) => Some(api),
        Ok(Err(e)) => {
            log::warn!("HidApi initialization failed: {e}");
            None
        }
        Err(_) => {
            log::warn!("HidApi initialization timed out after {timeout:?}");
            None
        }
    }
}

const HIDAPI_TIMEOUT: Duration = Duration::from_secs(5);
```

Then update `enumerate_all()` (line 56):

```rust
pub fn enumerate_all(&self) -> Vec<(String, DeviceInfo)> {
    let api = match new_hidapi_with_timeout(HIDAPI_TIMEOUT) {
        Some(api) => api,
        None => return Vec::new(),
    };
    // ... rest unchanged
}
```

Update `open_any()` (line 86):

```rust
pub fn open_any(&self) -> Result<Box<dyn StatusLightDevice>> {
    let api = new_hidapi_with_timeout(HIDAPI_TIMEOUT)
        .ok_or(StatusLightError::DeviceNotFound)?;
    // ... rest unchanged
}
```

Update `open()` (line 99):

```rust
pub fn open(
    &self,
    driver_id: &str,
    serial: Option<&str>,
) -> Result<Box<dyn StatusLightDevice>> {
    let api = new_hidapi_with_timeout(HIDAPI_TIMEOUT)
        .ok_or(StatusLightError::DeviceNotFound)?;
    // ... rest unchanged
}
```

**Step 2: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 3: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: Clean

**Step 4: Commit**

```bash
git add crates/statuslight-core/src/registry.rs
git commit -m "fix(core): add 5s timeout to HidApi initialization to prevent IOKit hang"
```

---

## Task 3: Fix SwiftUI process management — add timeout and proper cancellation

**Files:**
- Modify: `macos/StatusLight/StatusLightCLI.swift`

**Step 1: Rewrite `run()` with timeout and proper pipe handling**

Replace the `run()` method (lines 305-330) with a version that:
- Reads pipe data concurrently (not after waitUntilExit — prevents pipe deadlock)
- Enforces a 15-second timeout (kills the process if it hangs)
- Returns immediately on timeout with `("", false)`

```swift
/// Run the CLI binary with the given arguments and return (stdout, success).
private func run(_ arguments: [String]) async -> (String, Bool) {
    await withCheckedContinuation { continuation in
        DispatchQueue.global(qos: .userInitiated).async { [binaryPath] in
            let process = Process()
            process.executableURL = URL(fileURLWithPath: binaryPath)
            process.arguments = arguments

            let pipe = Pipe()
            process.standardOutput = pipe
            process.standardError = pipe

            // Collect output asynchronously to avoid pipe buffer deadlock.
            var outputData = Data()
            let outputLock = NSLock()
            pipe.fileHandleForReading.readabilityHandler = { handle in
                let chunk = handle.availableData
                if !chunk.isEmpty {
                    outputLock.lock()
                    outputData.append(chunk)
                    outputLock.unlock()
                }
            }

            do {
                try process.run()
            } catch {
                pipe.fileHandleForReading.readabilityHandler = nil
                continuation.resume(returning: ("", false))
                return
            }

            // Watchdog: kill process after 15 seconds.
            let processRef = process
            let watchdog = DispatchWorkItem {
                if processRef.isRunning {
                    processRef.terminate()
                }
            }
            DispatchQueue.global(qos: .utility).asyncAfter(
                deadline: .now() + 15.0,
                execute: watchdog
            )

            process.waitUntilExit()
            watchdog.cancel()

            // Stop reading and collect final data.
            pipe.fileHandleForReading.readabilityHandler = nil
            let finalChunk = pipe.fileHandleForReading.readDataToEndOfFile()

            outputLock.lock()
            outputData.append(finalChunk)
            let output = String(data: outputData, encoding: .utf8) ?? ""
            outputLock.unlock()

            let ok = process.terminationStatus == 0
            continuation.resume(returning: (output, ok))
        }
    }
}
```

**Step 2: Apply the same fix to `runWithStdin()`**

Replace the `runWithStdin()` method (lines 269-301) with the same timeout/pipe pattern:

```swift
/// Run the CLI binary with the given arguments, piping input to stdin.
private func runWithStdin(_ arguments: [String], input: String) async -> (String, Bool) {
    await withCheckedContinuation { continuation in
        DispatchQueue.global(qos: .userInitiated).async { [binaryPath] in
            let process = Process()
            process.executableURL = URL(fileURLWithPath: binaryPath)
            process.arguments = arguments

            let outPipe = Pipe()
            process.standardOutput = outPipe
            process.standardError = outPipe

            let inPipe = Pipe()
            process.standardInput = inPipe

            // Collect output asynchronously.
            var outputData = Data()
            let outputLock = NSLock()
            outPipe.fileHandleForReading.readabilityHandler = { handle in
                let chunk = handle.availableData
                if !chunk.isEmpty {
                    outputLock.lock()
                    outputData.append(chunk)
                    outputLock.unlock()
                }
            }

            do {
                try process.run()
                if let data = input.data(using: .utf8) {
                    inPipe.fileHandleForWriting.write(data)
                }
                inPipe.fileHandleForWriting.closeFile()
            } catch {
                outPipe.fileHandleForReading.readabilityHandler = nil
                continuation.resume(returning: ("", false))
                return
            }

            // Watchdog: kill process after 15 seconds.
            let processRef = process
            let watchdog = DispatchWorkItem {
                if processRef.isRunning {
                    processRef.terminate()
                }
            }
            DispatchQueue.global(qos: .utility).asyncAfter(
                deadline: .now() + 15.0,
                execute: watchdog
            )

            process.waitUntilExit()
            watchdog.cancel()

            outPipe.fileHandleForReading.readabilityHandler = nil
            let finalChunk = outPipe.fileHandleForReading.readDataToEndOfFile()

            outputLock.lock()
            outputData.append(finalChunk)
            let output = String(data: outputData, encoding: .utf8) ?? ""
            outputLock.unlock()

            let ok = process.terminationStatus == 0
            continuation.resume(returning: (output, ok))
        }
    }
}
```

**Step 3: Commit**

```bash
git add macos/StatusLight/StatusLightCLI.swift
git commit -m "fix(macos): add 15s process timeout and fix pipe deadlock in CLI wrapper"
```

---

## Task 4: Guard ViewModel polling against process accumulation

**Files:**
- Modify: `macos/StatusLight/StatusLightApp.swift`

**Step 1: Guard refresh() against re-entrant execution**

In `StatusLightApp.swift`, replace the `refresh()` method (~line 170) with a guarded version that skips if a previous refresh is still in flight:

```swift
private var isRefreshing = false

func refresh() {
    // Skip if a previous refresh hasn't completed yet.
    guard !isRefreshing else { return }
    isRefreshing = true
    refreshTask = Task { @MainActor in
        let dev = await cli.isDeviceConnected()
        let slack = await cli.isSlackConnected()
        guard !Task.isCancelled else {
            self.isRefreshing = false
            return
        }
        self.deviceConnected = dev
        self.slackConnected = slack
        self.isRefreshing = false
    }
}
```

**Step 2: Skip CLI calls if binary doesn't exist**

In `ViewModel.init()` (~line 147), guard the polling calls:

```swift
init() {
    _showInDock = Published(initialValue: UserDefaults.standard.bool(forKey: "showInDock"))
    _autoSyncSlack = Published(initialValue: UserDefaults.standard.bool(forKey: "autoSyncSlack"))
    let savedIntensity = UserDefaults.standard.double(forKey: "lightIntensity")
    _intensity = Published(initialValue: savedIntensity > 0 ? savedIntensity : 1.0)
    isInstalled = cli.isInstalled
    if isInstalled {
        startPolling()
        loadCustomPresets()
        startUpdateChecking()
    }
}
```

**Step 3: Commit**

```bash
git add macos/StatusLight/StatusLightApp.swift
git commit -m "fix(macos): guard polling against re-entrant execution and skip if not installed"
```

---

## Task 5: Add ThrottleInterval to LaunchAgent plist

**Files:**
- Modify: `crates/statuslight-cli/src/startup.rs:56-87`

**Step 1: Add ThrottleInterval and conditional KeepAlive**

In `startup.rs`, update the `plist_contents()` function to add a 30-second throttle and only restart on non-zero exit:

Replace the `KeepAlive` section (lines 75-76) in the plist template:

```rust
fn plist_contents(statuslightd_path: &str) -> String {
    let escaped_path = xml_escape(statuslightd_path);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{PLIST_LABEL}</string>

  <key>ProgramArguments</key>
  <array>
    <string>{escaped_path}</string>
  </array>

  <key>RunAtLoad</key>
  <true/>

  <key>KeepAlive</key>
  <dict>
    <key>SuccessfulExit</key>
    <false/>
  </dict>

  <key>ThrottleInterval</key>
  <integer>30</integer>

  <key>StandardOutPath</key>
  <string>/tmp/statuslight-daemon.log</string>

  <key>StandardErrorPath</key>
  <string>/tmp/statuslight-daemon.log</string>
</dict>
</plist>
"#
    )
}
```

**Step 2: Update affected tests**

The test `plist_has_keep_alive` (line 239) asserts `plist.contains("<key>KeepAlive</key>")` — this still passes since the key exists.

The test `plist_has_run_at_load` (line 232) asserts `<true/>` exists — now there are two `<true/>` tags (RunAtLoad and inside KeepAlive dict). The `contains` check still passes.

Add a new test to verify ThrottleInterval:

```rust
#[test]
fn plist_has_throttle_interval() {
    let plist = plist_contents("/usr/local/bin/statuslightd");
    assert!(plist.contains("<key>ThrottleInterval</key>"));
    assert!(plist.contains("<integer>30</integer>"));
}
```

**Step 3: Run tests**

Run: `cargo test -p statuslight-cli`
Expected: All pass

**Step 4: Commit**

```bash
git add crates/statuslight-cli/src/startup.rs
git commit -m "fix(cli): add ThrottleInterval and conditional KeepAlive to LaunchAgent plist"
```

---

## Task 6: Wrap blocking HID calls in spawn_blocking in daemon

**Files:**
- Modify: `crates/statuslight-daemon/src/api.rs:365-381`
- Modify: `crates/statuslight-daemon/src/button.rs:29-43`
- Modify: `crates/statuslight-daemon/src/main.rs:74-101`

**Step 1: Wrap get_devices handler**

In `api.rs`, update `get_devices()` (line 365):

```rust
async fn get_devices() -> Result<Json<Vec<DeviceEntry>>, (StatusCode, Json<ErrorResponse>)> {
    let all = tokio::task::spawn_blocking(|| {
        let registry = DeviceRegistry::with_builtins();
        registry.enumerate_all()
    })
    .await
    .unwrap_or_default();

    let entries: Vec<DeviceEntry> = all
        .into_iter()
        .map(|(_, d)| DeviceEntry {
            path: d.path,
            serial: d.serial,
            manufacturer: d.manufacturer,
            product: d.product,
            vid: format!("0x{:04x}", d.vid),
            pid: format!("0x{:04x}", d.pid),
            driver: d.driver_id,
        })
        .collect();
    Ok(Json(entries))
}
```

**Step 2: Wrap device reconnect in try_set_color**

In `api.rs`, update the reconnect block in `try_set_color()` (line 201):

```rust
if devices_guard.is_empty() {
    drop(devices_guard);
    let dev = tokio::task::spawn_blocking(|| {
        let registry = DeviceRegistry::with_builtins();
        registry.open_any()
    })
    .await
    .map_err(|_| map_error(StatusLightError::DeviceNotFound))?
    .map_err(map_error)?;
    devices_guard = state.inner.devices.lock().await;
    devices_guard.push(dev);
}
```

**Step 3: Wrap device reconnect in get_device_color**

In `api.rs`, update `get_device_color()` (line 390):

```rust
if devices_guard.is_empty() {
    drop(devices_guard);
    let dev = tokio::task::spawn_blocking(|| {
        let registry = DeviceRegistry::with_builtins();
        registry.open_any()
    })
    .await
    .map_err(|_| map_error(StatusLightError::DeviceNotFound))?
    .map_err(map_error)?;
    devices_guard = state.inner.devices.lock().await;
    devices_guard.push(dev);
}
```

**Step 4: Wrap button poll HID access in spawn_blocking**

In `button.rs`, update the device read block (lines 29-43):

```rust
let color = {
    let mut devices = state_clone.inner.devices.lock().await;
    if devices.is_empty() {
        drop(devices);
        let dev = tokio::task::spawn_blocking(|| {
            DeviceRegistry::with_builtins().open_any()
        })
        .await;
        match dev {
            Ok(Ok(d)) => {
                let mut devices = state_clone.inner.devices.lock().await;
                devices.push(d);
            }
            _ => continue,
        }
    }
    // get_color() is blocking HID I/O — but we hold the mutex,
    // and the timeout in protocol.rs is only 200ms, so this is
    // acceptable on a tokio thread. Moving it to spawn_blocking
    // would require Send on the device (HidDevice is Send but
    // extracting it from behind tokio::Mutex is complex).
    let devices = state_clone.inner.devices.lock().await;
    if devices.is_empty() {
        continue;
    }
    match devices[0].get_color() {
        Some(Ok(c)) => c,
        _ => continue,
    }
};
```

**Step 5: Wrap daemon startup HID access in spawn_blocking**

In `main.rs`, update lines 74-101:

```rust
// Enumerate and open all devices at startup (blocking HID I/O).
let opened_devices = tokio::task::spawn_blocking(|| {
    let registry = statuslight_core::DeviceRegistry::with_builtins();
    let all_devices = registry.enumerate_all();
    if all_devices.is_empty() {
        log::warn!("No devices at startup (will retry on requests)");
        return Vec::new();
    }
    let mut opened = Vec::new();
    for (driver_id, info) in &all_devices {
        let serial = info.serial.as_deref();
        match registry.open(driver_id, serial) {
            Ok(dev) => {
                log::info!(
                    "Opened device: {} (driver={}, serial={:?})",
                    dev.driver_name(),
                    driver_id,
                    serial
                );
                opened.push(dev);
            }
            Err(e) => {
                log::warn!(
                    "Failed to open device (driver={}, serial={:?}): {e}",
                    driver_id,
                    serial
                );
            }
        }
    }
    opened
})
.await
.unwrap_or_default();

if !opened_devices.is_empty() {
    let mut devices_guard = state.inner.devices.lock().await;
    for dev in opened_devices {
        devices_guard.push(dev);
    }
    log::info!("{} device(s) opened at startup", devices_guard.len());
}
```

**Step 6: Run clippy and tests**

Run: `cargo clippy --workspace -- -D warnings && cargo test --workspace`
Expected: Clean clippy, all tests pass

**Step 7: Commit**

```bash
git add crates/statuslight-daemon/src/api.rs crates/statuslight-daemon/src/button.rs crates/statuslight-daemon/src/main.rs
git commit -m "fix(daemon): wrap blocking HID calls in spawn_blocking to prevent tokio thread starvation"
```

---

## Task 7: Fix stale socket cleanup race

**Files:**
- Modify: `crates/statuslight-daemon/src/main.rs:53-57`

**Step 1: Replace TOCTOU pattern with unconditional remove**

Replace lines 53-57:

```rust
// Remove stale socket file if it exists.
if args.socket.exists() {
    std::fs::remove_file(&args.socket)
        .with_context(|| format!("failed to remove stale socket: {}", args.socket.display()))?;
}
```

with:

```rust
// Remove stale socket file (ignore if not found).
match std::fs::remove_file(&args.socket) {
    Ok(()) => log::debug!("Removed stale socket: {}", args.socket.display()),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
    Err(e) => {
        anyhow::bail!("failed to remove stale socket {}: {e}", args.socket.display());
    }
}
```

**Step 2: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: Clean

**Step 3: Commit**

```bash
git add crates/statuslight-daemon/src/main.rs
git commit -m "fix(daemon): fix stale socket cleanup TOCTOU race"
```

---

## Task 8: Build, install, and verify (NO binary execution on this machine)

**Files:**
- No code changes

**Step 1: Format and lint**

Run: `cargo fmt --all && cargo clippy --workspace -- -D warnings`
Expected: Clean

**Step 2: Run all tests**

Run: `cargo test --workspace`
Expected: All pass

**Step 3: Release build**

Run: `cargo build --workspace --release`
Expected: Succeeds

**Step 4: Build app bundle**

Run: `bash scripts/build-app.sh 0.3.1`
Expected: App bundle and DMG created

**Step 5: Install**

Run: `cp -R target/release/StatusLight.app /Applications/StatusLight.app`

**Step 6: Manual verification (user must do this)**

The user should:
1. Open Activity Monitor (to watch for runaway processes)
2. Double-click StatusLight.app in /Applications
3. Verify the app opens without freezing
4. Check Activity Monitor for accumulating `statuslight` processes (there should be none)

**Step 7: Commit and push**

```bash
git push
```
