# Quick Start

## CLI Usage

Plug in your Slicky and try these commands:

```bash
# Set to a named preset
statuslight set red
statuslight set available
statuslight set in-meeting

# Set to exact RGB values
statuslight rgb 255 128 0

# Set to a hex color
statuslight hex "#FF8000"
statuslight hex ff8000

# Turn off
statuslight off

# List all presets
statuslight presets

# List connected devices
statuslight devices
```

## Daemon Usage

Start the daemon to control the light over HTTP:

```bash
# Start on default socket
statuslightd

# Start with custom socket path
statuslightd --socket /tmp/statuslight.sock

# Start with Slack sync
statuslightd --slack-token xoxp-your-token --slack-interval 30
```

Control via curl:

```bash
# Check status
curl --unix-socket /tmp/statuslight.sock http://localhost/status

# Set color by preset name
curl --unix-socket /tmp/statuslight.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"color":"red"}' http://localhost/color

# Set by hex
curl --unix-socket /tmp/statuslight.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"color":"#FF8000"}' http://localhost/color

# Set by RGB
curl --unix-socket /tmp/statuslight.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"r":255,"g":128,"b":0}' http://localhost/rgb

# Turn off
curl --unix-socket /tmp/statuslight.sock -X POST http://localhost/off

# List presets
curl --unix-socket /tmp/statuslight.sock http://localhost/presets
```

## Slack Integration

Configure Slack to automatically sync your status light:

```bash
# Configure emoji-to-color mapping
curl --unix-socket /tmp/statuslight.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "token": "xoxp-your-token",
    "poll_interval_secs": 30,
    "emoji_map": {
      ":no_entry:": "#FF0000",
      ":calendar:": "#FF4500",
      ":palm_tree:": "#808080",
      ":house:": "#00FF00"
    }
  }' http://localhost/slack/configure

# Enable polling
curl --unix-socket /tmp/statuslight.sock -X POST http://localhost/slack/enable

# Check Slack status
curl --unix-socket /tmp/statuslight.sock http://localhost/slack/status

# Disable polling
curl --unix-socket /tmp/statuslight.sock -X POST http://localhost/slack/disable
```

Your Slack token needs the `users.profile:read` scope. Create one at [api.slack.com/apps](https://api.slack.com/apps).
