# Plan 017 — Bidirectional Slack Status Sync (Slack → Light)

## Context

The "Auto-sync status to Slack" toggle only syncs **light → Slack** (setting a preset updates Slack status). The reverse doesn't work — changing status/presence in Slack doesn't update the light. When the user goes "Away" or "Active" in Slack, the light should reflect that.

The daemon already has Socket Mode (real-time events via WebSocket) and emoji polling (60s). This plan adds:
1. **`user_change` event handling** — real-time custom status emoji → light sync
2. **Presence polling** — `users.getPresence` in the existing 60s poll → active/away light sync

## Architecture

```
Slack                          Daemon                         Light
─────                          ──────                         ─────
user_change event ──(Socket)──→ filter by user_id
                               extract status_emoji ─────────→ set color
                               map via emoji_colors

users.getPresence ←─(poll 60s)─ "away"  → away preset color ─→ set color
                               "active" → available color
```

**Priority:** event animation > custom status emoji > presence color

## Changes

### 1. Manifest updates

**`crates/statuslight-cli/src/slack.rs`** and **`docs/slack-setup/manifest.json`**:
- Add `users:read` to both user and bot scopes (required for `user_change` event and `users.getPresence`)
- Add `user_change` to `bot_events`

### 2. `crates/statuslight-daemon/src/state.rs`

Add `user_id: Option<String>` to `SlackState` struct + default `None` in constructor.

### 3. `crates/statuslight-daemon/src/main.rs`

After Slack state is populated, resolve user ID:
- Call `slack::resolve_user_id()` which calls `auth.test` with user token
- Store returned `user_id` in `SlackState`
- This filters `user_change` events (which fire for ALL workspace users)

### 4. `crates/statuslight-daemon/src/slack.rs`

**(A) New helper: `resolve_user_id()`**
- Calls `auth.test` with token, returns `user_id` string from response

**(B) Pass user_id + emoji_colors to Socket Mode**

`start_socket_mode()` clones `user_id` and `emoji_colors` along with `rules`, threading them through to `run_socket_loop()` → `handle_event()`.

**(C) Handle `user_change` in `handle_event()`**

- If event type == `"user_change"`, dispatch to `handle_user_change()`
- Filter by matching `event.user.id` against our `user_id`
- Skip if `event_animation_active`
- Extract `status_emoji` from `event.user.profile`
- Check `status_expiration` (reuse existing expiration logic)
- Map emoji → color via `emoji_colors`
- If color found → `set_device_color()`
- If no emoji match → return (presence poll handles the fallback)
- Logs a warning if `user_id` is `None` (auth.test failed at startup)

**(D) Add presence check to emoji poll**

In the existing poll loop, after `fetch_emoji_color()` returns `None`:
- Call new `fetch_presence()` helper
- `fetch_presence()` calls `users.getPresence` API with user_token
- Map `"away"` → `Preset::Away.color()`, `"active"` → `Preset::Available.color()`
- Set device color

## Files Modified

| File | Change |
|------|--------|
| `crates/statuslight-cli/src/slack.rs` | Add `users:read` scope (user + bot), `user_change` event to manifest |
| `docs/slack-setup/manifest.json` | Same manifest updates |
| `crates/statuslight-daemon/src/state.rs` | Add `user_id` field to `SlackState` |
| `crates/statuslight-daemon/src/main.rs` | Resolve user_id on startup via `auth.test` |
| `crates/statuslight-daemon/src/slack.rs` | Handle `user_change` events, add presence polling, new helpers |

## Verification

1. `cargo fmt --all && cargo clippy --workspace -- -D warnings && cargo test --workspace`
2. Rebuild + install via `scripts/build-app.sh`
3. Test: change custom status emoji in Slack → light updates in real-time
4. Test: set "Away" in Slack → light goes to away color within 60s
5. Test: set "Active" in Slack → light goes to available color within 60s
6. Test: DM flash animation still takes priority over status sync
7. Note: existing Slack apps need `users:read` user + bot scope + `user_change` event added manually, then reinstall to workspace
