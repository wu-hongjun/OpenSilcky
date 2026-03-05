# Architecture

## Crate Dependency Graph

```
slicky-core  <─── slicky-cli      (binary: "slicky")
             <─── slicky-daemon   (binary: "slickyd")
             <─── slicky-ffi      (staticlib + cdylib: "libslicky_ffi")
                        ^
                   Swift GUI (future — links via C bridging header)
```

`slicky-core` has zero internal workspace dependencies. All other crates depend only on `slicky-core`.

## slicky-core

The core library with no binary-specific dependencies. Contains:

| Module | Purpose |
|--------|---------|
| `error.rs` | `SlickyError` enum (via `thiserror`), `Result<T>` alias |
| `color.rs` | `Color` struct (RGB), `Preset` enum, hex parsing |
| `protocol.rs` | Wire constants, `build_set_color_report()` |
| `device.rs` | `SlickyDevice` trait, `HidSlickyDevice` impl, `DeviceInfo` |

### Design Decisions

- **`SlickyDevice` trait**: Enables mocking for tests. The trait has a default `off()` implementation.
- **Stateless device operations**: The CLI opens the device, sends one report, and drops it. No persistent connection needed.
- **BGR wire order**: The protocol uses BGR byte ordering. This is encapsulated in `protocol.rs` — callers always work with RGB.

## slicky-cli

Thin binary using `clap` derive macros. Each subcommand opens the device, performs one action, prints the result, and exits.

## slicky-daemon

An HTTP daemon using `axum` on a Unix domain socket.

### State Management

```rust
AppState {
    device: Mutex<Option<HidSlickyDevice>>,   // Mutex: HidDevice is !Sync
    current_color: Mutex<Option<Color>>,
    slack: Mutex<SlackState>,
}
```

- **`Option<HidSlickyDevice>`**: The daemon starts without a device and reconnects on each request if needed (USB hot-plug resilience).
- **`tokio::sync::Mutex`**: Required because `HidDevice` is `Send` but not `Sync`.

### Slack Integration

A background `tokio::spawn` task polls `users.profile.get` every N seconds, maps the status emoji to a color via the configured emoji map, and sets the device. Errors are logged but don't crash the daemon.

## slicky-ffi

C-callable functions for native GUI integration. Safety rules:

1. Every function wrapped in `catch_unwind`
2. Null pointer checks on all `*const c_char` parameters
3. Invalid UTF-8 returns error code `-5`
4. Each call is stateless — opens device, writes, closes
5. `cbindgen` auto-generates `slicky.h` at build time

## Repository Structure

```
OpenSilcky/
├── Cargo.toml                    # Workspace root
├── mkdocs.yml                    # Documentation config
├── docs/                         # MkDocs source
├── crates/
│   ├── slicky-core/              # Core library
│   ├── slicky-cli/               # CLI binary
│   ├── slicky-daemon/            # HTTP daemon
│   └── slicky-ffi/               # C FFI
└── macos/                        # Swift GUI (future)
    └── OpenSlicky/
```
