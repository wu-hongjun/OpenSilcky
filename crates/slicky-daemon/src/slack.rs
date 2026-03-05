//! Slack API integration for automatic status light sync.
//!
//! Polls `users.profile.get` to read the user's status emoji, then maps it
//! to a device color via the configured emoji map.

use std::collections::HashMap;
use std::time::Duration;

use slicky_core::{Color, HidSlickyDevice, SlickyDevice};

use crate::state::AppState;

/// Slack API endpoint for fetching the authenticated user's profile.
const PROFILE_URL: &str = "https://slack.com/api/users.profile.get";

/// Fetch the user's Slack status emoji and resolve it to a color.
///
/// Returns `Ok(Some(color))` if the status emoji matches a key in `emoji_map`,
/// `Ok(None)` if there is no match or no status set.
pub async fn fetch_slack_color(
    client: &reqwest::Client,
    token: &str,
    emoji_map: &HashMap<String, Color>,
) -> anyhow::Result<Option<Color>> {
    let resp: serde_json::Value = client
        .get(PROFILE_URL)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;

    if !resp["ok"].as_bool().unwrap_or(false) {
        let err_msg = resp["error"].as_str().unwrap_or("unknown error");
        anyhow::bail!("Slack API error: {err_msg}");
    }

    let profile = &resp["profile"];
    let emoji = match profile["status_emoji"].as_str() {
        Some(e) if !e.is_empty() => e,
        _ => return Ok(None),
    };

    // Check if the status has expired.
    if let Some(exp) = profile["status_expiration"].as_i64() {
        if exp > 0 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            if exp < now {
                return Ok(None);
            }
        }
    }

    Ok(emoji_map.get(emoji).copied())
}

/// Start a background tokio task that polls Slack every N seconds.
///
/// Aborts any previously running poll task first.
pub async fn start_polling(state: &AppState) {
    stop_polling(state).await;

    let mut slack = state.inner.slack.lock().await;
    let token = match &slack.token {
        Some(t) => t.clone(),
        None => return,
    };
    let interval_secs = slack.poll_interval_secs;
    let emoji_map = slack.emoji_map.clone();
    let state_clone = state.clone();

    let handle = tokio::spawn(async move {
        let client = reqwest::Client::new();
        loop {
            match fetch_slack_color(&client, &token, &emoji_map).await {
                Ok(Some(color)) => {
                    let mut device_guard = state_clone.inner.device.lock().await;
                    // Try to reconnect if needed.
                    if device_guard.is_none() {
                        if let Ok(dev) = HidSlickyDevice::open() {
                            *device_guard = Some(dev);
                        }
                    }
                    if let Some(dev) = device_guard.as_ref() {
                        if let Err(e) = dev.set_color(color) {
                            log::warn!("Slack poll: failed to set color: {e}");
                            *device_guard = None;
                        } else {
                            drop(device_guard);
                            *state_clone.inner.current_color.lock().await = Some(color);
                        }
                    }
                }
                Ok(None) => {
                    log::debug!("Slack poll: no matching status emoji");
                }
                Err(e) => {
                    log::warn!("Slack poll error: {e}");
                }
            }
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    });

    slack.enabled = true;
    slack.poll_handle = Some(handle);
}

/// Stop the background polling task, if running.
pub async fn stop_polling(state: &AppState) {
    let mut slack = state.inner.slack.lock().await;
    if let Some(handle) = slack.poll_handle.take() {
        handle.abort();
    }
    slack.enabled = false;
}

/// Default emoji-to-color mappings.
pub fn default_emoji_map() -> HashMap<String, Color> {
    let mut map = HashMap::new();
    map.insert(":no_entry:".to_string(), Color::new(255, 0, 0));
    map.insert(":red_circle:".to_string(), Color::new(255, 0, 0));
    map.insert(":calendar:".to_string(), Color::new(255, 69, 0));
    map.insert(":spiral_calendar_pad:".to_string(), Color::new(255, 69, 0));
    map.insert(":palm_tree:".to_string(), Color::new(128, 128, 128));
    map.insert(":house:".to_string(), Color::new(0, 255, 0));
    map.insert(":large_green_circle:".to_string(), Color::new(0, 255, 0));
    map
}
