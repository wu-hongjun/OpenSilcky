//! Shared application state for the Slicky daemon.

use std::collections::HashMap;
use std::sync::Arc;

use slicky_core::{Color, HidSlickyDevice};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Shared state passed to all axum route handlers.
#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

/// The inner state protected by `Arc`.
pub struct AppStateInner {
    /// The HID device handle. `Option` allows starting without a connected device.
    /// `Mutex` because `HidDevice` is `Send` but not `Sync`.
    pub device: Mutex<Option<HidSlickyDevice>>,
    /// The last color set on the device.
    pub current_color: Mutex<Option<Color>>,
    /// Slack integration state.
    pub slack: Mutex<SlackState>,
}

/// Slack polling configuration and runtime state.
pub struct SlackState {
    /// Whether Slack polling is active.
    pub enabled: bool,
    /// The Slack user token (`xoxp-...`).
    pub token: Option<String>,
    /// Polling interval in seconds (default 30).
    pub poll_interval_secs: u64,
    /// Mapping of Slack status emoji to device colors.
    pub emoji_map: HashMap<String, Color>,
    /// Handle to the background polling task, if running.
    pub poll_handle: Option<JoinHandle<()>>,
}

impl AppState {
    /// Create a new `AppState` with no device connected and Slack disabled.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                device: Mutex::new(None),
                current_color: Mutex::new(None),
                slack: Mutex::new(SlackState {
                    enabled: false,
                    token: None,
                    poll_interval_secs: 30,
                    emoji_map: HashMap::new(),
                    poll_handle: None,
                }),
            }),
        }
    }
}
