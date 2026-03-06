# StatusLight

Open-source driver and tools for the [Lexcelon Slicky](https://www.lexcelon.com/products/slicky) USB status light.

<img width="344" height="299" alt="Screenshot 2026-03-06 at 5 48 46 PM" src="https://github.com/user-attachments/assets/8e5818f4-c13c-4060-8a2e-4a478505465a" />


The Slicky is a USB-connected desk light that communicates your availability via color — red for busy, green for available, and any custom color you choose. Lexcelon stopped maintaining the original driver, so we built an open-source replacement in Rust.

> It turns out that very few people are using the software on mac and we don't have the capacity at the moment to maintain the program. We have plans to re-vamp the whole thing with more modern software but I'm unable to promise any sort of timeline on that at the moment. (Lexcelon Representive, June 2024)

## What's Included

| Crate | Description |
|-------|-------------|
| **statuslight-core** | Core library — color handling, HID protocol, device communication |
| **statuslight** (CLI) | Command-line tool to control the light |
| **statuslightd** (daemon) | HTTP daemon with REST API and Slack integration |
| **statuslight-ffi** | C FFI bindings for building native GUIs (Swift, etc.) |

## Install

Pre-built `.dmg` releases are available for **macOS Apple Silicon** (ARM64) on the [Releases](https://github.com/wu-hongjun/StatusLight/releases) page. Download, mount, and copy the binaries to a directory in your `PATH`.

For other platforms (macOS Intel, Linux), build from source:

```bash
# Install dependencies (macOS)
brew install hidapi

# Build
git clone https://github.com/wu-hongjun/StatusLight.git
cd StatusLight
cargo build --workspace --release

# Install the CLI
cargo install --path crates/statuslight-cli
```

```bash
# Set to red
statuslight set red

# Set to a custom hex color
statuslight hex "#FF8000"

# Turn off
statuslight off
```

## Features

- Set the light to any RGB color or named preset
- Control via CLI, HTTP API, or C FFI
- Automatic Slack status sync — your light matches your Slack status emoji
- USB hot-plug resilience — reconnects automatically
- macOS and Linux support

## Project Structure

```
StatusLight/
├── Cargo.toml                    # Workspace root
├── mkdocs.yml                    # Documentation config
├── docs/                         # MkDocs source
├── crates/
│   ├── statuslight-core/         # Core library
│   ├── statuslight-cli/          # CLI binary
│   ├── statuslight-daemon/       # HTTP daemon
│   └── statuslight-ffi/          # C FFI
└── macos/                        # Swift GUI (future)
    └── StatusLight/
```

## Documentation

Full docs are available at [wu-hongjun.github.io/StatusLight](https://wu-hongjun.github.io/StatusLight/).

## License

MIT
