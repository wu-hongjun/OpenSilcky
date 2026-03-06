# Plan 020 — Multi-Device Drivers, Brightness, and TCP API

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add 5 new device drivers (Luxafor, Blink1, BlinkStick, Kuando, Embrava), global brightness control, multi-device targeting, verbose device listing, and optional TCP API.

**Status:** Implemented.

## Summary of Changes

### Phase 1: Core Infrastructure
- Added `vid`, `pid`, `driver_id` fields to `DeviceInfo`
- Created shared `hid_helpers` module
- Refactored `SlickyDriver` to use `hid_helpers`
- Added `brightness` (0-100) and `DaemonConfig` to `Config`
- Added `UnknownDriver` error variant

### Phase 2: Multi-Device Support
- Daemon holds `Vec<Box<dyn StatusLightDevice>>` + `AtomicU8` brightness
- All API endpoints broadcast to all devices
- `?device=<serial>` query param for targeting
- `POST /brightness` endpoint
- CLI: `--all`, `--device`, `--brightness` global flags
- `DeviceProxy` supports `DirectMulti` variant
- `statuslight devices -v` shows VID/PID
- FFI: `statuslight_set_brightness`, `statuslight_get_brightness`, `statuslight_device_count`
- Optional TCP API via `--tcp-port` / config

### Phase 3-7: Device Drivers
- **Luxafor Flag** (0x04d8/0xf372): 8-byte HID write, RGB, `CMD_STEADY=0x01`
- **Blink(1)** (0x27b8/0x01ed): 9-byte feature report, `CMD=0x6E`
- **BlinkStick** (0x20a0/0x41e5): 4-byte feature report, **GRB** color order
- **Embrava Blynclight** (4 VID/PIDs): 9-byte HID write, **RBG** color order, footer `0xFF 0x22`
- **Kuando Busylight** (4 VID/PIDs): 64-byte HID write, **PWM 0-100** encoding, keepalive thread

### Phase 8: TCP API
- Implemented in Phase 2 (`--tcp-port`, `--tcp-bind`)

## New Files (7)
- `crates/statuslight-core/src/drivers/hid_helpers.rs`
- `crates/statuslight-core/src/drivers/luxafor.rs`
- `crates/statuslight-core/src/drivers/blink1.rs`
- `crates/statuslight-core/src/drivers/blinkstick.rs`
- `crates/statuslight-core/src/drivers/embrava.rs`
- `crates/statuslight-core/src/drivers/kuando.rs`
- `docs/plans/020-multi-device-drivers-brightness.md`
