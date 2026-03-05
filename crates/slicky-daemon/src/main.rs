mod api;
mod slack;
mod state;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use tokio::net::UnixListener;

use state::AppState;

#[derive(Parser)]
#[command(
    name = "slickyd",
    version,
    about = "HTTP daemon for Slicky USB status lights"
)]
struct Args {
    /// Path to the Unix domain socket.
    #[arg(long, default_value = "/tmp/slicky.sock")]
    socket: PathBuf,

    /// Slack user token for automatic status sync.
    #[arg(long)]
    slack_token: Option<String>,

    /// Slack polling interval in seconds.
    #[arg(long, default_value_t = 30)]
    slack_interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Remove stale socket file if it exists.
    if args.socket.exists() {
        std::fs::remove_file(&args.socket)
            .with_context(|| format!("failed to remove stale socket: {}", args.socket.display()))?;
    }

    let state = AppState::new();

    // Try to open the device at startup (non-fatal if not found).
    match slicky_core::HidSlickyDevice::open() {
        Ok(dev) => {
            log::info!("Slicky device found at startup");
            *state.inner.device.lock().await = Some(dev);
        }
        Err(e) => {
            log::warn!("No device at startup (will retry on requests): {e}");
        }
    }

    // Configure Slack if a token was provided.
    if let Some(token) = args.slack_token {
        let mut slack_state = state.inner.slack.lock().await;
        slack_state.token = Some(token);
        slack_state.poll_interval_secs = args.slack_interval;
        slack_state.emoji_map = slack::default_emoji_map();
        drop(slack_state);
        slack::start_polling(&state).await;
        log::info!(
            "Slack sync enabled (polling every {}s)",
            args.slack_interval
        );
    }

    let app = api::router(state.clone());

    let listener = UnixListener::bind(&args.socket)
        .with_context(|| format!("failed to bind socket: {}", args.socket.display()))?;
    log::info!("Listening on {}", args.socket.display());

    // Graceful shutdown on SIGINT / SIGTERM.
    let shutdown_signal = async {
        let ctrl_c = tokio::signal::ctrl_c();
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to register SIGTERM handler");
        tokio::select! {
            _ = ctrl_c => { log::info!("Received SIGINT, shutting down"); }
            _ = sigterm.recv() => { log::info!("Received SIGTERM, shutting down"); }
        }
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .context("server error")?;

    // Clean up socket file.
    let _ = std::fs::remove_file(&args.socket);

    // Stop Slack polling.
    slack::stop_polling(&state).await;

    log::info!("Daemon stopped");
    Ok(())
}
