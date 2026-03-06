# Plan 005 — UX Overhaul: Config, Slack OAuth, Startup, Auto-Update

## Context

The current StatusLight UX has several pain points:
- **Slack**: No OAuth. Users must manually obtain a `xoxp-` token and pass it as a CLI arg to `statuslightd` on every startup. No persistence.
- **Startup**: No launchd integration. Users must manually run `statuslightd` from terminal.
- **Updates**: No update checking. Users have no way to know a new version is available.

Goal: Make StatusLight "extremely simple" — one command to connect Slack, one command to enable startup, and automatic update notifications.

## Overview

Four features, each building on the previous:

1. **Config file** (statuslight-core) — persistent `~/.config/statuslight/config.toml`
2. **Slack OAuth** (statuslight-cli) — `statuslight slack login/logout/status`
3. **Startup management** (statuslight-cli) — `statuslight startup enable/disable/status`
4. **Auto-update** (statuslight-cli + statuslight-daemon) — `statuslight update check` + daemon auto-check

## 1. Config File (`statuslight-core`)

New module: `crates/statuslight-core/src/config.rs`

```toml
# ~/.config/statuslight/config.toml
[slack]
token = "xoxp-..."
poll_interval_secs = 30

[startup]
enabled = false

[updates]
auto_check = true
last_check = "2026-03-05T00:00:00Z"
latest_version = "0.2.0"
```

Key struct:
```rust
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub slack: SlackConfig,      // token, poll_interval_secs
    pub startup: StartupConfig,  // enabled
    pub updates: UpdateConfig,   // auto_check, last_check, latest_version
}
```

Methods: `Config::load()`, `Config::save()`, `Config::path()` (returns `~/.config/statuslight/config.toml`)

## 2. Slack Credentials & Secrets Strategy

**Slack App**: `A0AJS883QK0`

Secrets are baked into the release binary at compile time via `env!()` macros — never hardcoded in source. Two sources:

| Context | How secrets are provided |
|---------|------------------------|
| **Local dev** | `.env` file (already gitignored) loaded via `export $(cat .env \| xargs)` before `cargo build` |
| **CI release** | GitHub Secrets → env vars in the release workflow |

### `.env` file (local, gitignored)
Contains `SLACK_CLIENT_ID`, `SLACK_CLIENT_SECRET`, and `SLACK_SIGNING_SECRET`.

### GitHub Secrets to add (via repo Settings → Secrets)
- `SLACK_CLIENT_ID`
- `SLACK_CLIENT_SECRET`

### Release workflow update (`.github/workflows/release.yml`)
Env vars added to the build step so `env!()` macros resolve at compile time.

### In Rust code (`slack.rs`)
```rust
const SLACK_CLIENT_ID: &str = env!("SLACK_CLIENT_ID");
const SLACK_CLIENT_SECRET: &str = env!("SLACK_CLIENT_SECRET");
```

## 3. Slack OAuth Flow (`statuslight-cli`)

Module: `crates/statuslight-cli/src/slack.rs`

Slack App configured with `users.profile:read` user scope and redirect URL `http://127.0.0.1:19876/callback`.

### `statuslight slack login` flow:
1. Bind `TcpListener` on `127.0.0.1:19876`
2. Open browser to Slack OAuth authorize URL
3. User clicks "Allow" in Slack
4. Slack redirects to callback with `?code=XXXX`
5. Parse code from HTTP request line
6. Send "Success!" HTML response
7. POST `oauth.v2.access` with `{client_id, client_secret, code}` via `ureq`
8. Extract `authed_user.access_token` from response
9. Save to config, write config file

### `statuslight slack logout`: Remove token from config
### `statuslight slack status`: Show connection state with masked token

## 4. Startup Management (`statuslight-cli`)

Module: `crates/statuslight-cli/src/startup.rs`

### `statuslight startup enable`:
1. Find `statuslightd` binary (sibling of current exe, or `which statuslightd`)
2. Write LaunchAgent plist to `~/Library/LaunchAgents/com.statuslight.daemon.plist`
3. `launchctl load -w` the plist
4. Set `config.startup.enabled = true`

### `statuslight startup disable`:
1. `launchctl unload -w` the plist
2. Delete plist file
3. Set `config.startup.enabled = false`

### `statuslight startup status`: Show if enabled + if daemon is running

Plist features: `RunAtLoad=true`, `KeepAlive=true`, logs to `/tmp/statuslight-daemon.log`

## 5. Auto-Update (`statuslight-cli` + `statuslight-daemon`)

Modules: `crates/statuslight-cli/src/update.rs` (blocking/ureq), `crates/statuslight-daemon/src/update.rs` (async/reqwest)

- Hit `https://api.github.com/repos/wu-hongjun/StatusLight/releases/latest`
- Parse `tag_name`, compare with `CARGO_PKG_VERSION` via `semver`
- Rate-limit: at most once per 24h (check `config.updates.last_check`)
- If newer: log message with download URL (no auto-download)
- `statuslight update check`: manual check from CLI
- Daemon: `tokio::spawn` non-blocking check on startup if `config.updates.auto_check` is true

## 6. Daemon Changes (`statuslight-daemon/src/main.rs`)

Token priority: CLI arg > config file.
Spawns update check on startup.

## Dependencies Added

**Workspace `Cargo.toml`**: `toml`, `dirs`, `ureq` (with json feature), `open`, `semver`, `chrono`

| Crate | New deps |
|-------|----------|
| statuslight-core | `anyhow`, `toml`, `dirs`, `chrono` |
| statuslight-cli | `serde_json`, `ureq`, `open`, `semver`, `dirs`, `chrono` |
| statuslight-daemon | `semver`, `chrono` |

## Files Created/Modified

| File | Action |
|------|--------|
| `.env` | **New** — Slack secrets for local dev (gitignored) |
| `.github/workflows/release.yml` | Added env vars to build step |
| `Cargo.toml` | Added workspace deps |
| `crates/statuslight-core/Cargo.toml` | Added deps |
| `crates/statuslight-core/src/lib.rs` | Added `pub mod config; pub use config::Config;` |
| `crates/statuslight-core/src/config.rs` | **New** — Config struct, load/save |
| `crates/statuslight-cli/Cargo.toml` | Added deps |
| `crates/statuslight-cli/src/main.rs` | Added Slack/Startup/Update subcommands |
| `crates/statuslight-cli/src/slack.rs` | **New** — OAuth login/logout/status |
| `crates/statuslight-cli/src/startup.rs` | **New** — launchd enable/disable/status |
| `crates/statuslight-cli/src/update.rs` | **New** — blocking update check |
| `crates/statuslight-daemon/Cargo.toml` | Added deps |
| `crates/statuslight-daemon/src/main.rs` | Load config, token fallback, spawn update check |
| `crates/statuslight-daemon/src/update.rs` | **New** — async update check |
| `docs/plans/005-ux-overhaul.md` | **New** — this plan |

## Post-Merge Steps

After merging, add `SLACK_CLIENT_ID` and `SLACK_CLIENT_SECRET` as GitHub repository secrets (Settings → Secrets → Actions).

## Verification

1. `cargo build --workspace` — compiles clean
2. `cargo clippy --workspace -- -D warnings` — passes
3. `cargo test --workspace` — all tests pass
4. `statuslight slack login` opens browser, completes OAuth
5. `statuslight slack status` shows "logged in"
6. `statuslight startup enable` creates plist, daemon starts
7. `statuslight startup status` shows enabled + running
8. `statuslight startup disable` stops daemon, removes plist
9. `statuslight update check` prints current vs latest version
10. `statuslightd` reads token from config (no `--slack-token` needed)
