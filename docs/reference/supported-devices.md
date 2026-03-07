# Supported Devices

StatusLight supports **22 devices** across **8 driver families**. The Lexcelon Slicky is the officially supported device with full feature coverage including color readback. All other devices have community-level support — they use the correct HID protocol but have not been tested by the maintainer.

!!! tip
    Run `statuslight supported` in the CLI to see this table with live detection.

## Device Compatibility Table

### [Lexcelon Slicky](https://www.lexcelon.com/products/slicky) — Official

The Slicky is the primary device StatusLight was built for. It has full feature support including bidirectional communication (color readback).

| Device | VID | PID | Color Readback |
|--------|-----|-----|----------------|
| Slicky | `0x04d8` | `0xec24` | Yes |

### [Blink(1) by ThingM](https://blink1.thingm.com/) — Community

| Device | VID | PID |
|--------|-----|-----|
| blink(1) mk1 / mk2 / mk3 | `0x27b8` | `0x01ed` |

### [BlinkStick](https://www.blinkstick.com/) — Community

| Device | VID | PID |
|--------|-----|-----|
| BlinkStick / Strip / Nano / Flex / Square | `0x20a0` | `0x41e5` |

### [Embrava](https://www.embrava.com/products) — Community

| Device | VID | PID |
|--------|-----|-----|
| Blynclight | `0x2c0d` | `0x0001` |
| Blynclight Plus | `0x2c0d` | `0x0002` |
| Blynclight Mini | `0x2c0d` | `0x000a` |
| Blynclight (variant) | `0x2c0d` | `0x000c` |
| Blynclight Plus (variant) | `0x2c0d` | `0x0010` |
| Embrava Connect | `0x0e53` | `0x2516` |
| Embrava Connect Mini | `0x0e53` | `0x2517` |
| Plantronics Status Indicator (OEM) | `0x047f` | `0xd005` |

### [EPOS](https://www.eposaudio.com/en/us/products/ui-20-bl-usb-accessory-1000828) — Community

| Device | VID | PID |
|--------|-----|-----|
| Busylight (UI 20 BL) | `0x1395` | `0x0074` |

### [Kuando by Plenom](https://www.plenom.com/) — Community

| Device | VID | PID |
|--------|-----|-----|
| Busylight UC Alpha | `0x27bb` | `0x3bca` |
| Busylight Alpha (variant) | `0x27bb` | `0x3bcb` |
| Busylight UC Omega | `0x27bb` | `0x3bcd` |
| Busylight Omega (variant) | `0x27bb` | `0x3bce` |
| Busylight (variant) | `0x27bb` | `0x3bcf` |
| Busylight Alpha (Microchip VID) | `0x04d8` | `0xf848` |

### [Luxafor](https://luxafor.com/product/flag/) — Community

| Device | VID | PID |
|--------|-----|-----|
| Flag / Mute / Orb / Bluetooth | `0x04d8` | `0xf372` |

### [MuteMe](https://muteme.com/) — Community

| Device | VID | PID |
|--------|-----|-----|
| MuteMe Original | `0x16c0` | `0x27db` |
| MuteMe Original (variant) | `0x20a0` | `0x42da` |
| MuteMe Mini | `0x20a0` | `0x42db` |

## Support Levels

**Official** — Fully tested by the maintainer. All features supported including color readback, animations, and Slack integration.

**Community** — Uses the correct HID protocol based on public documentation and reverse engineering. Basic functionality (set color, turn off) should work, but the device has not been physically tested. If you have one of these devices and can confirm it works (or doesn't), please [open an issue](https://github.com/wu-hongjun/StatusLight/issues).

## USB Permissions (Linux)

On Linux, you need a udev rule to access HID devices without root. Create `/etc/udev/rules.d/99-statuslight.rules`:

```
# Lexcelon Slicky
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="04d8", ATTRS{idProduct}=="ec24", MODE="0666"

# Blink(1)
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="27b8", ATTRS{idProduct}=="01ed", MODE="0666"

# BlinkStick
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="20a0", ATTRS{idProduct}=="41e5", MODE="0666"

# Embrava
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="2c0d", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0e53", MODE="0666"

# EPOS
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1395", ATTRS{idProduct}=="0074", MODE="0666"

# Kuando
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="27bb", MODE="0666"

# Luxafor
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="04d8", ATTRS{idProduct}=="f372", MODE="0666"

# MuteMe
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="27db", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="20a0", ATTRS{idProduct}=="42da", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="20a0", ATTRS{idProduct}=="42db", MODE="0666"
```

Then reload:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```
