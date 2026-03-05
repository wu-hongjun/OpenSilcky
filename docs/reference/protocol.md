# HID Protocol Reference

The Lexcelon Slicky-1.0 is a USB HID device that accepts vendor-specific output reports to control its LED color.

## Device Identification

| Field | Value |
|-------|-------|
| Vendor ID | `0x04D8` (Microchip Technology) |
| Product ID | `0xEC24` |
| Manufacturer | Lexcelon |
| Product | Slicky-1.0 |

## Report Format

The device accepts a 64-byte HID output report. When sent via the HID API, the buffer is 65 bytes — a report ID byte (always `0x00`) followed by the 64-byte payload.

### Set Color Report (65 bytes)

```
Index: [0]   [1]   [2]   [3]   [4]   [5]   [6]   [7]   [8]   [9..64]
Value: 0x00  0x0A  0x04  0x00  0x00  0x00  BLUE  GRN   RED   0x00...
       ^^^^  ^^^^  ^^^^                    ^^^^  ^^^^  ^^^^
       rpt   cmd   sub                     B     G     R
       ID
```

| Offset | Size | Description |
|--------|------|-------------|
| 0 | 1 | Report ID — always `0x00` |
| 1 | 1 | Command — `0x0A` (set color) |
| 2 | 1 | Subcommand — `0x04` |
| 3–5 | 3 | Reserved — `0x00` |
| 6 | 1 | **Blue** channel (0–255) |
| 7 | 1 | **Green** channel (0–255) |
| 8 | 1 | **Red** channel (0–255) |
| 9–64 | 56 | Padding — `0x00` |

!!! warning "BGR byte order"
    The color bytes are in **BGR** order, not RGB. Blue is at index 6, green at 7, red at 8.

### Turn Off

To turn the light off, send a set-color report with R=0, G=0, B=0. The command and subcommand bytes remain the same.

## Communication Pattern

- **Write-only**: The host sends output reports to the device. No input reports are read.
- **Stateless**: Each report sets the current color. There is no handshake or session.
- **Single report**: One 65-byte write per color change. No multi-report sequences.
- **No persistent connection required**: The device can be opened, written to, and closed for each operation.

## Constants

```rust
pub const VENDOR_ID: u16 = 0x04D8;
pub const PRODUCT_ID: u16 = 0xEC24;
pub const REPORT_SIZE: usize = 64;   // HID report payload
pub const BUFFER_SIZE: usize = 65;   // report ID + payload
```

## Reverse Engineering Notes

The protocol was reverse-engineered by capturing USB traffic from the original Lexcelon desktop application using Wireshark with USBPcap. The key observations:

1. All communication uses a single HID output report
2. The command byte `0x0A` with subcommand `0x04` is the only observed command
3. Color bytes are at fixed offsets 6, 7, 8 in BGR order
4. All other bytes in the payload are zero
5. The device responds immediately to each report — no acknowledgment needed
