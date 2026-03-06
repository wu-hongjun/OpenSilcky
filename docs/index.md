# StatusLight

**Open-source driver and tools for the [Lexcelon Slicky](https://www.lexcelon.com/products/slicky) USB status light.**

The Slicky is a USB-connected desk light that communicates your availability via color — red for busy, green for available, and any custom color you choose. Lexcelon stopped maintaining the original driver, so we built an open-source replacement in Rust.

## What's Included

| Component | Description |
|-----------|-------------|
| **statuslight-core** | Core library — color handling, HID protocol, device communication |
| **statuslight** (CLI) | Command-line tool to control the light |
| **statuslightd** (daemon) | HTTP daemon with REST API and Slack integration |
| **statuslight-ffi** | C FFI bindings for building native GUIs (Swift, etc.) |

## Features

- Set the light to any RGB color or named preset
- Control via CLI, HTTP API, or C FFI
- Automatic Slack status sync — your light matches your Slack status emoji
- USB hot-plug resilience — reconnects automatically
- macOS and Linux support

## Quick Example

```bash
# Set to red
statuslight set red

# Set to a custom hex color
statuslight hex "#FF8000"

# Turn off
statuslight off

# List presets
statuslight presets
```

## Getting Started

Head to the [Installation](getting-started/installation.md) guide to get set up.
