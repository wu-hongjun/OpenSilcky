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
git clone https://github.com/openslicky/openslicky.git
cd openslicky
cargo build --workspace --release
```

The binaries are placed in `target/release/`:

| Binary | Description |
|--------|-------------|
| `slicky` | CLI tool |
| `slickyd` | HTTP daemon |

The FFI library is at `target/release/libslicky_ffi.a` (static) and `target/release/libslicky_ffi.dylib` (dynamic).

## Install the CLI

```bash
cargo install --path crates/slicky-cli
```

This installs `slicky` to `~/.cargo/bin/`.

## Verify

```bash
slicky devices
```

If your Slicky is plugged in, you'll see its serial number and product info. If not, you'll see "No Slicky devices found" — that's expected.

## USB Permissions (Linux)

On Linux, you may need a udev rule to access the device without root:

```bash
# /etc/udev/rules.d/99-slicky.rules
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="04d8", ATTRS{idProduct}=="ec24", MODE="0666"
```

Then reload:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```
