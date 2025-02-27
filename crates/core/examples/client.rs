use std::{error::Error, sync::Arc};

use liveview_native_core::{
    callbacks::{Issuer, LiveChannelStatus, NetworkEventHandler},
    client::config::{LiveViewClientConfiguration, Platform},
    dom::ffi::Document,
    live_socket::LiveChannel,
    LiveViewClient,
};
use phoenix_channels_client::{EventPayload, Socket, SocketStatus};

#[cfg(target_os = "android")]
const DEFAULT_HOST: &str = "10.0.2.2:4001";

#[cfg(not(target_os = "android"))]
const DEFAULT_HOST: &str = "127.0.0.1:4001";

struct NetworkLogger;

impl NetworkEventHandler for NetworkLogger {
    fn handle_event(&self, payload: EventPayload) {
        match payload.payload {
            phoenix_channels_client::Payload::JSONPayload { json } => {
                let json = serde_json::Value::from(json);
                log::info!(
                    "Payload received: \n {}",
                    serde_json::to_string_pretty(&json).unwrap()
                );
            }
            phoenix_channels_client::Payload::Binary { .. } => {
                log::info!("Binary Payload received",);
            }
        }
    }

    fn handle_channel_status_change(&self, status: LiveChannelStatus) {
        log::info!("View channel status update: {status:?}");
    }

    fn handle_socket_status_change(&self, status: SocketStatus) {
        log::info!("Socket status update: {status:?}");
    }

    fn handle_view_reloaded(
        &self,
        issuer: Issuer,
        document: Arc<Document>,
        _: Arc<LiveChannel>,
        _: Arc<Socket>,
        _: bool,
    ) {
        log::info!("Reload: issuer: {issuer:?} \n {}", document.render());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = format!("http://{DEFAULT_HOST}/stream");

    let config = LiveViewClientConfiguration {
        format: Platform::Swiftui,
        network_event_handler: Some(Arc::new(NetworkLogger)),
        log_to_stdout: true,
        ..Default::default()
    };

    let _client = LiveViewClient::initial_connect(config, url, Default::default()).await?;

    log::info!("Ctrl + C to exit");

    tokio::signal::ctrl_c().await?;

    Ok(())
}
