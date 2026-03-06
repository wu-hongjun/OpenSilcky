# CLI Reference

**Binary:** `statuslight`

```
statuslight <COMMAND>
```

## Commands

### `statuslight set <NAME>`

Set the light to a named preset.

```bash
statuslight set red
statuslight set available
statuslight set in-meeting    # hyphens allowed
statuslight set InMeeting     # PascalCase also works
```

**Output:** `Set to red (#FF0000)`

### `statuslight rgb <R> <G> <B>`

Set the light to exact RGB values (0–255 each).

```bash
statuslight rgb 255 128 0
```

**Output:** `Set to RGB(255, 128, 0) #FF8000`

### `statuslight hex <COLOR>`

Set the light to a hex color.

```bash
statuslight hex "#FF8000"
statuslight hex FF8000
statuslight hex f80           # 3-char shorthand
```

**Output:** `Set to #FF8000`

### `statuslight off`

Turn the light off.

**Output:** `Light off`

### `statuslight presets`

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

### `statuslight devices`

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
