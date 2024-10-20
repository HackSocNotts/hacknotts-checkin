use checkin_output::CheckinOutput;
use clap::{ArgGroup, Parser};
use env_logger::Env;
use error::HackNottsCheckinError;
use futures_util::StreamExt;
use log::{debug, error, info};
use stdout_checkin_output::StdOutCheckinOutput;
use tito_types::{Checkin, NewCheckinBody, Ticket, WebhookCheckin};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

mod checkin_output;
mod error;
mod stdout_checkin_output;
mod tito_types;
mod typst_checkin_output;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("recheck")
        .args(&["recheck_id", "tito_slug"])
        .multiple(true)
        .required(false)
        .requires_all(&["recheck_id", "tito_slug"])
))]
struct Args {
    /// Reference to recheck. For example, JFUD-1
    #[arg(short, long, group = "recheck")]
    reference: Option<String>,

    /// Tito Checkin API slug, required for rechecking.
    #[arg(short, long, group = "recheck")]
    tito_slug: Option<String>,
}

async fn recheck(reference: &str, tito_slug: &str) -> Result<(), reqwest::Error> {
    info!("Rechecking {reference}");

    let checkins = reqwest::get(format!(
        "https://checkin.tito.io/checkin_lists/{tito_slug}/checkins"
    ))
    .await?
    .json::<Vec<Checkin>>()
    .await?;

    debug!("Fetched {} checkins", checkins.len());

    let tickets = reqwest::get(format!(
        "https://checkin.tito.io/checkin_lists/{tito_slug}/tickets"
    ))
    .await?
    .json::<Vec<Ticket>>()
    .await?;

    debug!("Fetched {} tickets", tickets.len());

    let wrapped_ticket = tickets.iter().find(|ticket| ticket.reference == reference);

    if wrapped_ticket.is_none() {
        error!("Failed to find a ticket with reference {reference}. Check Tito to ensure the attendee has a ticket and retry with correct reference.");
        return Ok(());
    }

    // We check if this is none above, we can get away with a cheeky unwrap ;)
    // Rust please add guard let statements I don't wanna enter indent hell 🥺
    let ticket = wrapped_ticket.unwrap();

    let is_checked_in = checkins
        .iter()
        .any(|checkin| checkin.ticket_id == ticket.id);

    if is_checked_in {
        info!("Checkin found! Reprinting ticket");
        StdOutCheckinOutput::checkin(ticket);
    } else {
        info!("Ticket is not checked in. Checking in and reprinting ticket.");

        let client = reqwest::Client::new();

        client
            .post(format!(
                "https://checkin.tito.io/checkin_lists/{tito_slug}/checkins"
            ))
            .json(&NewCheckinBody::new(ticket.id))
            .send()
            .await?;

        info!("Checkin successfully created! Printing ticket.");

        StdOutCheckinOutput::checkin(ticket);
    }

    Ok(())
}

async fn run_loop(message: Message) -> Result<(), HackNottsCheckinError> {
    let message_string = message.to_text()?;

    debug!("Received message from WebSocket");
    debug!("{message_string}");

    let checkin: WebhookCheckin = serde_json::from_str(message_string)?;

    info!("Processing checkin {}", checkin.id);

    StdOutCheckinOutput::checkin(&checkin);

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(); // what a line

    let args = Args::parse();

    if let (Some(recheck_id), Some(tito_slug)) = (args.reference, args.tito_slug) {
        // If the recheck arguments were specified, recheck and don't continue on to the runloop
        recheck(&recheck_id, &tito_slug)
            .await
            .expect("Failed to recheck!");
        return;
    }

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
