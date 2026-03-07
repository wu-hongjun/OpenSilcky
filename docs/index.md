# StatusLight

**Open-source driver and tools for USB status lights.** Officially supports the [Lexcelon Slicky](https://www.lexcelon.com/products/slicky), with community support for [7 additional device families](reference/supported-devices.md).

A USB status light communicates your availability via color — red for busy, green for available, and any custom color you choose. StatusLight provides a unified interface to control these lights from the command line, an HTTP API, or a native macOS app.

## What's Included

| Component | Description |
|-----------|-------------|
| **statuslight-core** | Core library — color handling, HID protocol, device communication |
| **statuslight** (CLI) | Command-line tool to control the light |
| **statuslightd** (daemon) | HTTP daemon with REST API and Slack integration |
| **statuslight-ffi** | C FFI bindings for building native GUIs (Swift, etc.) |

## Features

- Set the light to any RGB color or named preset
- Multi-device support — control any of [22 USB status lights](reference/supported-devices.md) from 8 manufacturers
- Control via CLI, HTTP API, native macOS app, or C FFI
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
