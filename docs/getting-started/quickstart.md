# Quick Start

## CLI Usage

Plug in your Slicky and try these commands:

```bash
# Set to a named preset
slicky set red
slicky set available
slicky set in-meeting

# Set to exact RGB values
slicky rgb 255 128 0

# Set to a hex color
slicky hex "#FF8000"
slicky hex ff8000

# Turn off
slicky off

# List all presets
slicky presets

# List connected devices
slicky devices
```

## Daemon Usage

Start the daemon to control the light over HTTP:

```bash
# Start on default socket
slickyd

# Start with custom socket path
slickyd --socket /tmp/slicky.sock

# Start with Slack sync
slickyd --slack-token xoxp-your-token --slack-interval 30
```

Control via curl:

```bash
# Check status
curl --unix-socket /tmp/slicky.sock http://localhost/status

# Set color by preset name
curl --unix-socket /tmp/slicky.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"color":"red"}' http://localhost/color

# Set by hex
curl --unix-socket /tmp/slicky.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"color":"#FF8000"}' http://localhost/color

# Set by RGB
curl --unix-socket /tmp/slicky.sock -X POST \
  -H 'Content-Type: application/json' \
  -d '{"r":255,"g":128,"b":0}' http://localhost/rgb

# Turn off
curl --unix-socket /tmp/slicky.sock -X POST http://localhost/off

# List presets
curl --unix-socket /tmp/slicky.sock http://localhost/presets
```

## Slack Integration

Configure Slack to automatically sync your status light:

```bash
# Configure emoji-to-color mapping
curl --unix-socket /tmp/slicky.sock -X POST \
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
curl --unix-socket /tmp/slicky.sock -X POST http://localhost/slack/enable

# Check Slack status
curl --unix-socket /tmp/slicky.sock http://localhost/slack/status

# Disable polling
curl --unix-socket /tmp/slicky.sock -X POST http://localhost/slack/disable
```

Your Slack token needs the `users.profile:read` scope. Create one at [api.slack.com/apps](https://api.slack.com/apps).
