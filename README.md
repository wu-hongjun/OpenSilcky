# OpenSlicky

Open-source driver and tools for the [Lexcelon Slicky](https://www.lexcelon.com/products/slicky) USB status light.

The Slicky is a USB-connected desk light that communicates your availability via color — red for busy, green for available, and any custom color you choose. Lexcelon stopped maintaining the original driver, so we built an open-source replacement in Rust.

## What's Included

| Crate | Description |
|-------|-------------|
| **slicky-core** | Core library — color handling, HID protocol, device communication |
| **slicky** (CLI) | Command-line tool to control the light |
| **slickyd** (daemon) | HTTP daemon with REST API and Slack integration |
| **slicky-ffi** | C FFI bindings for building native GUIs (Swift, etc.) |

## Quick Start

```bash
# Install dependencies (macOS)
brew install hidapi

# Build
git clone https://github.com/wu-hongjun/OpenSilcky.git
cd OpenSilcky
cargo build --workspace --release

# Install the CLI
cargo install --path crates/slicky-cli
```

```bash
# Set to red
slicky set red

# Set to a custom hex color
slicky hex "#FF8000"

# Turn off
slicky off
```

## Features

- Set the light to any RGB color or named preset
- Control via CLI, HTTP API, or C FFI
- Automatic Slack status sync — your light matches your Slack status emoji
- USB hot-plug resilience — reconnects automatically
- macOS and Linux support

## Project Structure

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

## Documentation

Full docs are available at [wu-hongjun.github.io/OpenSilcky](https://wu-hongjun.github.io/OpenSilcky/).

## License

MIT
