# References

Third-party projects referenced for feature parity and device compatibility research.
These repos are git-ignored and not shipped with StatusLight.

## busylight

- **Source:** <https://github.com/JnyJny/busylight>
- **Language:** Python (wraps `busylight-core`)
- **Purpose:** CLI + HTTP API for USB presence/status lights
- **Why referenced:** Comprehensive device support list and feature set we can learn from

### Supported Devices (23 devices, 9 vendors)

| Vendor | Device | VID | PID | Interface | LEDs | Notes |
|--------|--------|-----|-----|-----------|------|-------|
| Agile Innovative | BlinkStick | 0x20a0 | 0x41e5 | HID | Multi | Also Pro, Square, Strip, Nano, Flex variants (same VID/PID) |
| ThingM | Blink(1) mk2/mk3 | 0x27b8 | 0x01ed | HID | 2 | Top/bottom LEDs individually addressable |
| Embrava | Blynclight | 0x2c0d | 0x0001 | HID | 1 | 9-byte packets |
| Embrava | Blynclight Mini | 0x2c0d | 0x000a | HID | 1 | |
| Embrava | Blynclight Plus | 0x2c0d | 0x000c | HID | 1 | |
| Embrava | Blynclight (alt) | 0x0e53 | 0x2517 | HID | 1 | Alternate VID |
| Plantronics | Status Indicator | 0x047f | 0xd005 | HID | 1 | Rebranded Blynclight Plus |
| Kuando | Busylight Alpha | 0x04d8 | 0xf848 | HID | 1 | 64-byte packets, requires keepalive |
| Kuando | Busylight Alpha (alt) | 0x27bb | 0x3bca | HID | 1 | Alternate VID |
| Kuando | Busylight Omega | 0x27bb | 0x3bcd | HID | 1 | Supports audio/ringtones |
| Kuando | Busylight Omega (alt) | 0x27bb | 0x3bcf | HID | 1 | |
| Luxafor | Flag | 0x04d8 | 0xf372 | HID | Multi | 8-byte packets; supports fade, strobe, wave, pattern |
| Luxafor | Mute | 0x04d8 | 0xf372 | HID | 1 | Same VID/PID as Flag |
| Luxafor | Orb | 0x04d8 | 0xf372 | HID | 1 | Same VID/PID as Flag |
| Luxafor | Bluetooth | 0x04d8 | 0xf372 | HID | 1 | Same VID/PID as Flag |
| MuteMe | Original (proto) | 0x16c0 | 0x27db | HID | 1 | 2-byte packets, 3-bit color, touch button |
| MuteMe | Original (prod) | 0x20a0 | 0x42da | HID | 1 | |
| MuteMe | Mini | 0x20a0 | 0x42db | HID | 1 | |
| Compulab | fit-statUSB | 0x2047 | 0x03df | Serial | 1 | Plain text commands via USB CDC |
| Busy Tag | Busy Tag | 0x303a | 0x81df | Serial | 1 | USB CDC serial |

### Features We Don't Have Yet

| Feature | busylight | StatusLight | Gap |
|---------|-----------|-------------|-----|
| Static color (RGB/hex/named) | Yes | Yes | -- |
| Named presets | Yes | Yes | -- |
| Custom presets (user-defined) | Yes | Yes | -- |
| Blink animation | Yes | Yes (flash) | -- |
| Pulse/breathing animation | Yes | Yes | -- |
| Rainbow animation | Yes | Yes | -- |
| Transition animation | No | Yes | We have more |
| SOS animation | No | Yes | We have more |
| Cycle animation | No | Yes | We have more |
| Brightness/dimming (`--dim`) | Yes | No | **Gap** |
| Multi-LED targeting (`--led`) | Yes | No | **Gap** |
| HTTP REST API server | Yes | Yes (daemon) | Partial — ours is Unix socket only |
| List connected devices | Yes (`list`) | Yes (`devices`) | -- |
| Device info (VID/PID/serial) | Yes (`list -v`) | Partial | **Gap** — could show more HW detail |
| Multi-device targeting (`--light-id`) | Yes | No | **Gap** |
| Control all devices at once (`--all`) | Yes | No | **Gap** |
| Web API authentication | Yes (Basic) | No | **Gap** (low priority) |
| CORS support | Yes | No | **Gap** (low priority) |
| udev rules generation (Linux) | Yes | N/A | macOS only for now |
| Keepalive packets | Yes (Kuando) | No | **Gap** (needed for Kuando) |
| Serial device support | Yes | No | **Gap** — HID only currently |
| Multi-vendor device support | Yes (9 vendors) | No (Slicky only) | **Gap** — our driver architecture supports it |
| Async/background effects | Yes | Yes (daemon) | -- |
| Slack integration | No | Yes | We have more |
| macOS menu bar app | No | Yes | We have more |
| LaunchAgent auto-start | No | Yes | We have more |
| Config file (TOML) | No | Yes | We have more |
| Self-update mechanism | No | Yes | We have more |
| Color override per-preset | No | Yes | We have more |

### Priority Features to Add

1. **Brightness/dimming** — Scale RGB output by a percentage (0-100%). Simple to implement in `Color`.
2. **Multi-device targeting** — Our `DeviceRegistry` already supports this; need CLI flags (`--all`, `--device`).
3. **More device drivers** — Start with popular ones: Luxafor Flag, Blink(1), BlinkStick, Kuando Busylight.
4. **Verbose device listing** — Show VID, PID, serial, driver name in `statuslight devices -v`.
5. **HTTP API over TCP** — Expose daemon API on a TCP port (not just Unix socket) for network access.
6. **Serial device support** — Add serial transport alongside HID for Compulab/Busy Tag devices.
7. **Keepalive support** — Required for Kuando Busylight devices that time out without periodic packets.

### Device Driver Priority (by popularity/availability)

1. **Luxafor Flag** — Very popular, widely available, multi-LED
2. **Blink(1)** — Well-known, open-source hardware, 2 LEDs
3. **BlinkStick** — Popular maker device, multi-LED variants
4. **Kuando Busylight** — Enterprise standard, needs keepalive
5. **Embrava Blynclight** — Common in corporate environments
6. **MuteMe** — Niche but interesting (touch button input)
7. **Compulab fit-statUSB** — Requires serial transport (lower priority)
