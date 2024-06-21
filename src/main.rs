use env_logger::Env;
use error::HackNottsCheckinError;
use futures_util::StreamExt;
use log::{debug, error, info};
use tito_types::Checkin;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

mod error;
mod tito_types;

async fn run_loop(message: Message) -> Result<(), HackNottsCheckinError> {
    let message_string = message.to_text()?;

    debug!("Received message from WebSocket");
    debug!("{message_string}");

    let checkin: Checkin = serde_json::from_str(message_string)?;

    info!("Processing checkin {checkin}");

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init(); // what a line

    let url = "wss://wh2ws.hacksoc.net/websocket/checkin-created";

    info!("Connecting to {url}");
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    info!("WebSocket handshake has been successfully completed");

    while let Some(Ok(message)) = ws_stream.next().await {
        if let Err(e) = run_loop(message).await {
            error!("Error occured when processing response: {e}");
        }
    }
}
