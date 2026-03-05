# OpenSlicky — Full Stack Scaffold Plan

## Context

We reverse-engineered the Lexcelon Slicky-1.0 USB status light's HID protocol (VID `0x04D8`, PID `0xEC24`). The device accepts 64-byte vendor-specific HID reports: `[0x00, 0x0A, 0x04, 0x00, 0x00, 0x00, B, G, R, ...]`. The company stopped maintaining the driver. We're building an open-source replacement in Rust, scaffolding the full stack in one go but implementing in dependency order.

---

## Coding Rules & Standards

### Rust Version & Toolchain
- **Edition**: 2021
- **Toolchain**: stable (currently rustc 1.93.0)
- **MSRV**: Not pinned yet — use stable features only, no nightly

### Formatting & Linting
- Run `cargo fmt --all` before every commit — no exceptions
- Run `cargo clippy --workspace -- -D warnings` — treat all warnings as errors
- No `#[allow(clippy::...)]` without a comment explaining why

### Error Handling
- **`slicky-core`**: Define a crate-specific `SlickyError` enum using `thiserror`. All public functions return `Result<T, SlickyError>`. Expose a type alias `pub type Result<T> = std::result::Result<T, SlickyError>;`
- **`slicky-cli`**: Use `anyhow::Result` at the binary level. Convert `SlickyError` to user-friendly messages via `.context()`.
- **`slicky-daemon`**: Use `anyhow::Result` internally. Map errors to appropriate HTTP status codes (400 for bad input, 503 for device not found, 500 for unexpected errors). Always return JSON error bodies: `{"error": "message"}`.
- **`slicky-ffi`**: Never panic across the FFI boundary. Catch all errors and return integer codes. Use `std::panic::catch_unwind` around any function that could panic.
- **Never use `.unwrap()` or `.expect()` in library code** (`slicky-core`). Binary crates may use `.expect()` only during startup initialization where failure is unrecoverable.

### Naming Conventions
- **Crates**: `slicky-*` (kebab-case)
- **Modules**: `snake_case` (Rust default)
- **Types**: `PascalCase` — e.g., `Color`, `SlickyDevice`, `HidSlickyDevice`
- **Functions**: `snake_case` — e.g., `set_color`, `from_hex`, `build_set_color_report`
- **Constants**: `SCREAMING_SNAKE_CASE` — e.g., `VENDOR_ID`, `BUFFER_SIZE`
- **FFI exports**: `slicky_` prefix on all `extern "C"` functions — e.g., `slicky_set_rgb`
- **Enum variants**: `PascalCase` — e.g., `Preset::InMeeting`, `SlickyError::DeviceNotFound`

### Documentation
- All public types and functions must have a `///` doc comment
- Module-level `//!` doc comments in each `lib.rs` and at the top of each module file
- Include `# Examples` in doc comments for `Color::from_hex`, `Preset::from_name`, and other user-facing APIs
- Protocol details (byte layout, constants) must be documented in `protocol.rs` with inline comments

### Testing
- Unit tests go in the same file as the code under test, in a `#[cfg(test)] mod tests {}` block
- Integration tests (if needed) go in `crates/<crate>/tests/`
- Test all public API surfaces in `slicky-core`:
  - `color.rs`: hex parsing (valid 6-char, valid 3-char, with/without `#`, invalid strings, edge cases like "000000" and "FFFFFF")
  - `color.rs`: preset lookup (all names, case insensitivity, hyphenated names, unknown names)
  - `protocol.rs`: report building (verify exact byte positions for R/G/B, verify off report is all zeros in color bytes)
  - `device.rs`: cannot unit test HID calls — manual testing only (document this)
- Tests must not require a physical device connected. Device tests are manual.
- Use `assert_eq!` with descriptive messages: `assert_eq!(result, expected, "from_hex should parse uppercase hex")`

### Dependencies
- Pin workspace dependencies in root `Cargo.toml` under `[workspace.dependencies]`
- Crate `Cargo.toml` files reference workspace deps: `hidapi.workspace = true`
- Minimize dependency count. Prefer std library solutions where reasonable.
- No `unsafe` code except in `slicky-ffi` (where it's required for FFI). All `unsafe` blocks must have a `// SAFETY:` comment.

### Git & Commits
- Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
- One logical change per commit
- PR branches: `feat/description` or `fix/description`

### Release Profile
```toml
[profile.release]
lto = "fat"
codegen-units = 1
strip = true
```

---

## Repository Structure

```
OpenSilcky/
├── Cargo.toml                    # Workspace root
├── .gitignore
├── README.md
├── docs/
│   └── plans/                    # Implementation plans
├── crates/
│   ├── slicky-core/              # Core library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # pub mod declarations, crate-level docs
│   │       ├── error.rs          # SlickyError enum (thiserror), Result alias
│   │       ├── color.rs          # Color struct, Preset enum, hex parsing
│   │       ├── protocol.rs       # Wire constants, build_set_color_report()
│   │       └── device.rs         # SlickyDevice trait, HidSlickyDevice impl
│   ├── slicky-cli/               # CLI binary ("slicky")
│   │   ├── Cargo.toml
│   │   └── src/main.rs           # clap derive, command dispatch
│   ├── slicky-daemon/            # HTTP daemon ("slickyd")
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # tokio entry, Unix socket bind, graceful shutdown
│   │       ├── api.rs            # axum Router, route handlers, request/response types
│   │       ├── state.rs          # AppState (Mutex-wrapped device + Slack state)
│   │       └── slack.rs          # Slack API polling, emoji-to-color mapping
│   └── slicky-ffi/               # C FFI for Swift GUI
│       ├── Cargo.toml            # crate-type = ["cdylib", "staticlib"]
│       ├── cbindgen.toml         # C header generation config
│       ├── build.rs              # Runs cbindgen to produce slicky.h
│       └── src/lib.rs            # extern "C" functions with panic catching
└── macos/                        # Swift/SwiftUI app (future phase, not in this scaffold)
    └── OpenSlicky/
```

---

## Dependency Graph

```
slicky-core  ←─── slicky-cli      (binary: "slicky")
             ←─── slicky-daemon   (binary: "slickyd")
             ←─── slicky-ffi      (staticlib + cdylib: "libslicky_ffi")
                        ↑
                   Swift GUI (links via C bridging header — future phase)
```

No circular dependencies. `slicky-core` has zero internal workspace dependencies.

---

## Workspace Dependencies (root Cargo.toml)

```toml
[workspace.dependencies]
# Shared across crates
anyhow = "1"
thiserror = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
env_logger = "0.11"

# Device communication
hidapi = "2"

# CLI
clap = { version = "4", features = ["derive"] }

# Daemon
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal", "net"] }
axum = "0.8"
hyper-util = { version = "0.1", features = ["tokio"] }
tower = "0.5"
reqwest = { version = "0.12", features = ["json"] }
```

### Per-crate dependencies

| Crate | Runtime deps | Build deps |
|-------|-------------|------------|
| `slicky-core` | `hidapi`, `thiserror`, `serde`, `serde_json`, `log` | — |
| `slicky-cli` | `slicky-core`, `clap`, `anyhow`, `env_logger`, `log` | — |
| `slicky-daemon` | `slicky-core`, `axum`, `tokio`, `hyper-util`, `tower`, `reqwest`, `serde`, `serde_json`, `anyhow`, `env_logger`, `log`, `clap` | — |
| `slicky-ffi` | `slicky-core`, `log`, `env_logger` | `cbindgen = "0.27"` |

---

## Detailed Type & Function Specifications

### `slicky-core/src/error.rs`

```rust
#[derive(Debug, thiserror::Error)]
pub enum SlickyError {
    #[error("no Slicky device found (VID=0x04D8, PID=0xEC24)")]
    DeviceNotFound,

    #[error("multiple Slicky devices found ({count}); specify a serial number")]
    MultipleDevices { count: usize },

    #[error("USB HID error: {0}")]
    Hid(#[from] hidapi::HidError),

    #[error("invalid hex color: {0}")]
    InvalidHexColor(String),

    #[error("unknown preset: {0}")]
    UnknownPreset(String),

    #[error("device write failed: expected {expected} bytes, got {actual}")]
    WriteMismatch { expected: usize, actual: usize },
}

pub type Result<T> = std::result::Result<T, SlickyError>;
```

### `slicky-core/src/color.rs`

```rust
/// An RGB color with each channel 0-255.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8 }

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self;
    pub const fn off() -> Self;               // (0, 0, 0)
    pub fn from_hex(s: &str) -> Result<Self>;  // "#FF0000", "FF0000", "f00"
    pub fn to_hex(&self) -> String;            // "#FF0000"
    pub fn is_off(&self) -> bool;
}
impl Display for Color { ... }                // "#RRGGBB"

/// Named presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Preset {
    Red, Green, Blue, Yellow, Cyan, Magenta, White, Orange, Purple,
    Available, Busy, Away, InMeeting,
}

impl Preset {
    pub fn color(&self) -> Color;              // maps variant to RGB
    pub fn all() -> &'static [Preset];         // all variants in order
    pub fn from_name(s: &str) -> Result<Self>; // case-insensitive, allows hyphens
    pub fn name(&self) -> &'static str;        // lowercase display name
}
```

**Preset color values:**
| Preset | R | G | B | Hex |
|--------|---|---|---|-----|
| Red | 255 | 0 | 0 | #FF0000 |
| Green | 0 | 255 | 0 | #00FF00 |
| Blue | 0 | 0 | 255 | #0000FF |
| Yellow | 255 | 255 | 0 | #FFFF00 |
| Cyan | 0 | 255 | 255 | #00FFFF |
| Magenta | 255 | 0 | 255 | #FF00FF |
| White | 255 | 255 | 255 | #FFFFFF |
| Orange | 255 | 165 | 0 | #FFA500 |
| Purple | 128 | 0 | 128 | #800080 |
| Available | 0 | 255 | 0 | #00FF00 |
| Busy | 255 | 0 | 0 | #FF0000 |
| Away | 255 | 255 | 0 | #FFFF00 |
| InMeeting | 255 | 69 | 0 | #FF4500 |

### `slicky-core/src/protocol.rs`

```rust
pub const VENDOR_ID: u16 = 0x04D8;
pub const PRODUCT_ID: u16 = 0xEC24;
pub const REPORT_SIZE: usize = 64;        // HID report payload
pub const BUFFER_SIZE: usize = 65;         // report ID (1) + payload (64)

// Byte offsets within the 65-byte buffer
const IDX_REPORT_ID: usize = 0;           // always 0x00
const IDX_COMMAND: usize = 1;             // 0x0A = set color
const IDX_SUBCOMMAND: usize = 2;          // 0x04
const IDX_BLUE: usize = 6;
const IDX_GREEN: usize = 7;
const IDX_RED: usize = 8;

const CMD_SET_COLOR: u8 = 0x0A;
const SUBCMD_SET_COLOR: u8 = 0x04;

/// Build the 65-byte HID output report for setting a color.
pub fn build_set_color_report(color: Color) -> [u8; BUFFER_SIZE];

/// Build the off report (RGB all zeros).
pub fn build_off_report() -> [u8; BUFFER_SIZE];
```

**Wire format (65 bytes):**
```
Index: [0]   [1]   [2]   [3]   [4]   [5]   [6]   [7]   [8]   [9..64]
Value: 0x00  0x0A  0x04  0x00  0x00  0x00  BLUE  GRN   RED   0x00...
       ^^^^  ^^^^  ^^^^                    ^^^^  ^^^^  ^^^^
       rpt   cmd   sub                     B     G     R
       ID
```

### `slicky-core/src/device.rs`

```rust
/// Trait for controlling a Slicky device. Enables mocking in tests.
pub trait SlickyDevice {
    fn set_color(&self, color: Color) -> Result<()>;
    fn off(&self) -> Result<()> { self.set_color(Color::off()) }
}

/// Real HID-backed device.
pub struct HidSlickyDevice { /* hidapi::HidDevice inside */ }

impl HidSlickyDevice {
    pub fn open() -> Result<Self>;                     // open first device found
    pub fn open_serial(serial: &str) -> Result<Self>;  // open by serial number
    pub fn enumerate() -> Result<Vec<DeviceInfo>>;     // list all connected devices
}

impl SlickyDevice for HidSlickyDevice { ... }

/// Info about a connected device (from enumeration).
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub path: String,
    pub serial: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}
```

**Device lifecycle**: The CLI opens the device, sends one report, and drops it. No persistent connection. The daemon holds a `Mutex<Option<HidSlickyDevice>>` and reconnects on each operation if needed (USB hot-plug resilience).

---

### `slicky-cli/src/main.rs`

**Binary name:** `slicky`

```rust
#[derive(Parser)]
#[command(name = "slicky", version, about = "Control your Slicky USB status light")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set light to a named preset (e.g., red, busy, available, in-meeting)
    Set { name: String },
    /// Set light to exact RGB values (0-255 each)
    Rgb { r: u8, g: u8, b: u8 },
    /// Set light to a hex color (#RRGGBB or RRGGBB)
    Hex { color: String },
    /// Turn the light off
    Off,
    /// List all available preset names and their colors
    Presets,
    /// List connected Slicky devices
    Devices,
}
```

**Output format:** Single line confirmation for actions, tabular output for listings. No colors/emoji in output (pipe-friendly). Exit code 0 on success, 1 on error.

**Examples:**
```
$ slicky set red
Set to red (#FF0000)

$ slicky rgb 255 128 0
Set to RGB(255, 128, 0) #FF8000

$ slicky hex "#FF8000"
Set to #FF8000

$ slicky off
Light off

$ slicky presets
NAME           COLOR
----------------------------
red            #FF0000
green          #00FF00
...

$ slicky devices
Device 1:
  Serial:       77971799
  Manufacturer: Lexcelon
  Product:      Slicky-1.0
```

---

### `slicky-daemon/src/api.rs` — Full API Specification

**All requests/responses are JSON. Content-Type: application/json.**

#### `GET /status`
Returns current daemon state.
```json
// Response 200
{
  "device_connected": true,
  "current_color": { "r": 255, "g": 0, "b": 0, "hex": "#FF0000" },
  "slack_sync_enabled": false
}
// Response 200 (no device)
{
  "device_connected": false,
  "current_color": null,
  "slack_sync_enabled": false
}
```

#### `POST /color`
Set by preset name or hex string.
```json
// Request
{ "color": "red" }       // preset name
{ "color": "#FF8000" }   // hex color

// Response 200
{ "color": { "r": 255, "g": 0, "b": 0, "hex": "#FF0000" } }

// Response 400
{ "error": "unknown preset: foobar" }

// Response 503
{ "error": "no Slicky device found" }
```

#### `POST /rgb`
Set by exact RGB values.
```json
// Request
{ "r": 255, "g": 128, "b": 0 }

// Response 200
{ "color": { "r": 255, "g": 128, "b": 0, "hex": "#FF8000" } }
```

#### `POST /off`
Turn off the light. No request body.
```json
// Response 200
{ "color": { "r": 0, "g": 0, "b": 0, "hex": "#000000" } }
```

#### `GET /presets`
List all available presets.
```json
// Response 200
[
  { "name": "red", "hex": "#FF0000" },
  { "name": "green", "hex": "#00FF00" },
  ...
]
```

#### `GET /devices`
List connected Slicky devices.
```json
// Response 200
[
  {
    "path": "DevSrvsID:4298190949",
    "serial": "77971799",
    "manufacturer": "Lexcelon",
    "product": "Slicky-1.0"
  }
]
```

#### `GET /slack/status`
```json
// Response 200
{
  "enabled": false,
  "poll_interval_secs": 30,
  "has_token": false,
  "emoji_map": {}
}
```

#### `POST /slack/configure`
```json
// Request
{
  "token": "xoxp-...",
  "poll_interval_secs": 30,
  "emoji_map": {
    ":no_entry:": "#FF0000",
    ":calendar:": "#FF4500",
    ":palm_tree:": "#808080"
  }
}

// Response 200
{ "enabled": true, "poll_interval_secs": 30 }
```

#### `POST /slack/enable`
Start polling (token must already be configured). No request body.
```json
// Response 200
{ "enabled": true }
// Response 400
{ "error": "no Slack token configured" }
```

#### `POST /slack/disable`
Stop polling. No request body.
```json
// Response 200
{ "enabled": false }
```

---

### `slicky-daemon/src/state.rs`

```rust
#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub device: Mutex<Option<HidSlickyDevice>>,  // Mutex because HidDevice is !Sync
    pub current_color: Mutex<Option<Color>>,
    pub slack: Mutex<SlackState>,
}

pub struct SlackState {
    pub enabled: bool,
    pub token: Option<String>,
    pub poll_interval_secs: u64,                  // default 30
    pub emoji_map: HashMap<String, Color>,
    pub poll_handle: Option<JoinHandle<()>>,       // abort on reconfigure
}
```

**Thread safety**: `HidDevice` is `Send` but not `Sync`. The `tokio::sync::Mutex` ensures only one task accesses the device at a time. The `Option` allows the daemon to start without a connected device and detect one later.

---

### `slicky-daemon/src/slack.rs`

```rust
/// Fetch the user's Slack profile and extract status emoji.
pub async fn fetch_slack_color(
    client: &reqwest::Client,
    token: &str,
    emoji_map: &HashMap<String, Color>,
) -> anyhow::Result<Option<Color>>;

/// Start a background tokio task that polls Slack every N seconds.
/// Aborts any previously running poll task.
pub async fn start_polling(state: AppState);

/// Stop the polling task.
pub async fn stop_polling(state: AppState);

/// Default emoji-to-color mappings.
pub fn default_emoji_map() -> HashMap<String, Color>;
```

**Slack API used:** `GET https://slack.com/api/users.profile.get` with `Authorization: Bearer xoxp-...`
**Required scope:** `users.profile:read`
**Rate limiting:** Slack allows ~1 req/sec; polling every 30s is well within limits (2,880 calls/day).

**Behavior:**
- If the status emoji matches a key in `emoji_map`, set the light to that color
- If no match or no status set, leave the light unchanged (manual control takes precedence)
- If the status has a `status_expiration` in the past, treat as no status
- Log errors but don't crash — retry on next poll cycle

---

### `slicky-ffi/src/lib.rs`

```rust
/// All FFI functions return i32:
///   0  = success
///  -1  = device not found
///  -2  = multiple devices found
///  -3  = HID communication error
///  -4  = invalid color value
///  -5  = invalid argument (null pointer, bad UTF-8)
///  -6  = write failed

#[no_mangle]
pub extern "C" fn slicky_init();
    // Initialize logging. Safe to call multiple times.

#[no_mangle]
pub extern "C" fn slicky_set_rgb(r: u8, g: u8, b: u8) -> i32;

#[no_mangle]
pub extern "C" fn slicky_set_hex(hex: *const c_char) -> i32;
    // SAFETY: caller must pass a valid null-terminated UTF-8 C string

#[no_mangle]
pub extern "C" fn slicky_set_preset(name: *const c_char) -> i32;
    // SAFETY: caller must pass a valid null-terminated UTF-8 C string

#[no_mangle]
pub extern "C" fn slicky_off() -> i32;

#[no_mangle]
pub extern "C" fn slicky_is_connected() -> i32;
    // Returns 1 if connected, 0 if not. Never returns error codes.
```

**FFI Safety Rules:**
1. Every `extern "C"` function must be wrapped in `std::panic::catch_unwind`. If a panic occurs, return error code -3.
2. All pointer parameters must be checked for null before dereferencing.
3. Use `CStr::from_ptr` to convert C strings — handle invalid UTF-8 by returning -5.
4. Never store Rust objects across the FFI boundary (no opaque pointers in this API).
5. Each call opens and closes the device (stateless). This is intentional — simplicity over performance.

**cbindgen.toml:**
```toml
language = "C"
include_guard = "SLICKY_FFI_H"
header = "/* Generated by cbindgen — do not edit manually */"
```

---

## Implementation Steps (in order)

### Step 1: Workspace scaffold
Create all files and directories. Every crate should have a minimal compilable state (empty `main()` or `lib.rs` with module declarations).

**Files to create:**
- `/Cargo.toml` — workspace root
- `/.gitignore` — Rust defaults + macOS + IDE
- `/crates/slicky-core/Cargo.toml`
- `/crates/slicky-core/src/lib.rs` — `pub mod` declarations
- `/crates/slicky-core/src/error.rs` — empty placeholder
- `/crates/slicky-core/src/color.rs` — empty placeholder
- `/crates/slicky-core/src/protocol.rs` — empty placeholder
- `/crates/slicky-core/src/device.rs` — empty placeholder
- `/crates/slicky-cli/Cargo.toml`
- `/crates/slicky-cli/src/main.rs` — `fn main() {}`
- `/crates/slicky-daemon/Cargo.toml`
- `/crates/slicky-daemon/src/main.rs` — `fn main() {}`
- `/crates/slicky-daemon/src/api.rs` — empty
- `/crates/slicky-daemon/src/state.rs` — empty
- `/crates/slicky-daemon/src/slack.rs` — empty
- `/crates/slicky-ffi/Cargo.toml`
- `/crates/slicky-ffi/cbindgen.toml`
- `/crates/slicky-ffi/build.rs`
- `/crates/slicky-ffi/src/lib.rs` — empty

**Verify:** `cargo check --workspace` succeeds.

### Step 2: `slicky-core` — implement all modules
Implement in this order (each builds on the prior):
1. `error.rs` — `SlickyError` enum, `Result` alias
2. `color.rs` — `Color` struct with all methods, `Preset` enum with all methods, unit tests
3. `protocol.rs` — constants, `build_set_color_report()`, `build_off_report()`, unit tests
4. `device.rs` — `SlickyDevice` trait, `HidSlickyDevice`, `DeviceInfo`

**Verify:** `cargo test -p slicky-core` passes. All color/protocol tests green.

### Step 3: `slicky-cli` — full CLI implementation
Implement clap derive structs and all command handlers. Each handler: open device, perform action, print result, handle errors with `.context()`.

**Verify:** `cargo build -p slicky-cli` produces working `slicky` binary. Manual test: `slicky set red`, `slicky off`, `slicky presets`, `slicky devices`.

### Step 4: `slicky-ffi` — C FFI layer
Implement all `extern "C"` functions with panic catching. Set up cbindgen build script.

**Verify:** `cargo build -p slicky-ffi` produces `libslicky_ffi.a` and `libslicky_ffi.dylib`. `slicky.h` is generated. Run `nm target/debug/libslicky_ffi.a | grep slicky_` to confirm exported symbols.

### Step 5: `slicky-daemon` — HTTP daemon with Slack
Implement in this order:
1. `state.rs` — `AppState`, `SlackState`
2. `api.rs` — axum router, all route handlers, request/response serde types
3. `slack.rs` — `fetch_slack_color`, `start_polling`, `stop_polling`, `default_emoji_map`
4. `main.rs` — clap args, socket bind, state initialization, graceful shutdown

**Verify:** `cargo build -p slicky-daemon`. Start with `slickyd`, test with:
```bash
curl --unix-socket /tmp/slicky.sock http://localhost/status
curl --unix-socket /tmp/slicky.sock -X POST -H 'Content-Type: application/json' -d '{"color":"red"}' http://localhost/color
curl --unix-socket /tmp/slicky.sock -X POST http://localhost/off
curl --unix-socket /tmp/slicky.sock http://localhost/presets
```

### Step 6: Final verification
```bash
cargo fmt --all --check          # formatting
cargo clippy --workspace -- -D warnings   # linting
cargo build --workspace          # all crates compile
cargo test --workspace           # all tests pass
```

---

## CLI Usage (end result)

```
slicky set red              # preset color
slicky set available        # status preset (green)
slicky set in-meeting       # hyphenated preset
slicky rgb 255 128 0        # exact RGB
slicky hex "#FF8000"        # hex color (quotes for shell)
slicky hex ff8000           # hex without # prefix
slicky off                  # turn off
slicky presets              # list all presets with hex colors
slicky devices              # list connected Slicky devices
```

---

## Daemon Usage (end result)

```bash
# Start daemon (foreground)
slickyd --socket /tmp/slicky.sock

# Start with Slack sync
slickyd --slack-token xoxp-... --slack-interval 30

# Control via curl
curl --unix-socket /tmp/slicky.sock http://localhost/status
curl --unix-socket /tmp/slicky.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"color":"red"}' http://localhost/color
curl --unix-socket /tmp/slicky.sock -X POST http://localhost/off

# Configure Slack via API
curl --unix-socket /tmp/slicky.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"token":"xoxp-...","emoji_map":{":no_entry:":"#FF0000"}}' \
  http://localhost/slack/configure
curl --unix-socket /tmp/slicky.sock -X POST http://localhost/slack/enable
```

---

## FFI Header (generated by cbindgen)

```c
#ifndef SLICKY_FFI_H
#define SLICKY_FFI_H

/* Generated by cbindgen — do not edit manually */

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void    slicky_init(void);
int32_t slicky_set_rgb(uint8_t r, uint8_t g, uint8_t b);
int32_t slicky_set_hex(const char *hex);
int32_t slicky_set_preset(const char *name);
int32_t slicky_off(void);
int32_t slicky_is_connected(void);

#ifdef __cplusplus
}
#endif

#endif /* SLICKY_FFI_H */
```

---

## Future Work (not in this scaffold)

- **macOS GUI**: Swift/SwiftUI app at `macos/OpenSlicky/` linking `libslicky_ffi.a`
- **Config file**: `~/.config/slicky/config.toml` for persisting Slack token, emoji map, socket path
- **launchd plist**: Auto-start daemon on login
- **Homebrew formula**: `brew install openslicky`
- **Windows/Linux support**: Test hidapi on other platforms, conditional compilation if needed
- **Slack Events API**: Replace polling with real-time WebSocket (Socket Mode) for instant status sync
