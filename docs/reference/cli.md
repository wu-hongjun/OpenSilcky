# CLI Reference

**Binary:** `slicky`

```
slicky <COMMAND>
```

## Commands

### `slicky set <NAME>`

Set the light to a named preset.

```bash
slicky set red
slicky set available
slicky set in-meeting    # hyphens allowed
slicky set InMeeting     # PascalCase also works
```

**Output:** `Set to red (#FF0000)`

### `slicky rgb <R> <G> <B>`

Set the light to exact RGB values (0–255 each).

```bash
slicky rgb 255 128 0
```

**Output:** `Set to RGB(255, 128, 0) #FF8000`

### `slicky hex <COLOR>`

Set the light to a hex color.

```bash
slicky hex "#FF8000"
slicky hex FF8000
slicky hex f80           # 3-char shorthand
```

**Output:** `Set to #FF8000`

### `slicky off`

Turn the light off.

**Output:** `Light off`

### `slicky presets`

List all available presets with their hex colors.

```
NAME           COLOR
----------------------------
red            #FF0000
green          #00FF00
blue           #0000FF
yellow         #FFFF00
cyan           #00FFFF
magenta        #FF00FF
white          #FFFFFF
orange         #FFA500
purple         #800080
available      #00FF00
busy           #FF0000
away           #FFFF00
in-meeting     #FF4500
```

### `slicky devices`

List connected Slicky devices.

```
Device 1:
  Serial:       77971799
  Manufacturer: Lexcelon
  Product:      Slicky-1.0
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Error (device not found, invalid input, HID error) |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Logging level (e.g., `debug`, `info`, `warn`) |
