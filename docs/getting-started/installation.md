# Installation

## Prerequisites

- **Rust toolchain** (stable, 1.70+): Install via [rustup](https://rustup.rs/)
- **hidapi system library**: Required for USB HID communication

### macOS

```bash
brew install hidapi
```

### Ubuntu / Debian

```bash
sudo apt install libhidapi-dev
```

### Fedora

```bash
sudo dnf install hidapi-devel
```

## Build from Source

```bash
git clone https://github.com/wu-hongjun/StatusLight.git
cd StatusLight
cargo build --workspace --release
```

The binaries are placed in `target/release/`:

| Binary | Description |
|--------|-------------|
| `statuslight` | CLI tool |
| `statuslightd` | HTTP daemon |

The FFI library is at `target/release/libstatuslight_ffi.a` (static) and `target/release/libstatuslight_ffi.dylib` (dynamic).

## Install the CLI

```bash
cargo install --path crates/statuslight-cli
```

This installs `statuslight` to `~/.cargo/bin/`.

## Verify

```bash
statuslight devices
```

If your Slicky is plugged in, you'll see its serial number and product info. If not, you'll see "No Slicky devices found" — that's expected.

## USB Permissions (Linux)

On Linux, you may need a udev rule to access the device without root:

```bash
# /etc/udev/rules.d/99-statuslight.rules
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="04d8", ATTRS{idProduct}=="ec24", MODE="0666"
```

Then reload:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```
