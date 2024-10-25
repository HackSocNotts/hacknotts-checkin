use std::path::PathBuf;

use checkin_output::CheckinOutput;
use clap::{ArgGroup, Parser};
use env_logger::Env;
use error::HackNottsCheckinError;
use futures_util::StreamExt;
use log::{debug, error, info};
use tito_types::{AdminTicketResponse, Checkin, NewCheckinBody, Ticket, WebhookCheckin};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use typst_checkin_output::TypstCheckinOutput;

mod checkin_output;
mod error;
mod stdout_checkin_output;
mod tito_types;
mod typst_checkin_output;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("recheck")
        .args(&["reference", "account_slug", "account_token", "checkin_slug", "event_slug"])
        .multiple(true)
        .required(false)
        .requires_all(&["reference", "account_slug", "account_token", "checkin_slug", "event_slug"])
))]
struct Args {
    /// The path to the ticket's Typst template.
    #[arg(long)]
    template_path: PathBuf,

    /// Reference to recheck. For example, JFUD-1.
    #[arg(long, group = "recheck")]
    reference: Option<String>,

    /// Tito Account slug, required for rechecking.
    #[arg(long, group = "recheck")]
    account_slug: Option<String>,

    /// Tito API token, required for rechecking.
    #[arg(long, group = "recheck")]
    account_token: Option<String>,

    /// Tito Checkin API slug, required for rechecking.
    #[arg(long, group = "recheck")]
    checkin_slug: Option<String>,

    /// Tito Checkin API slug, required for rechecking.
    #[arg(long, group = "recheck")]
    event_slug: Option<String>,
}

async fn recheck(
    reference: &str,
    account_slug: &str,
    account_token: &str,
    checkin_slug: &str,
    event_slug: &str,
    checkin_output: &impl CheckinOutput,
) -> Result<(), reqwest::Error> {
    info!("Rechecking {reference}");

    let client = reqwest::Client::new();

    let checkins = client
        .get(format!(
            "https://checkin.tito.io/checkin_lists/{checkin_slug}/checkins"
        ))
        .send()
        .await?
        .json::<Vec<Checkin>>()
        .await?;

    debug!("Fetched {} checkins", checkins.len());

    let tickets = client
        .get(format!(
            "https://checkin.tito.io/checkin_lists/{checkin_slug}/tickets"
        ))
        .send()
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
        info!("Checkin found! Fetching full ticket details");

        let ticket = client
            .get(format!(
                "https://api.tito.io/v3/{account_slug}/{event_slug}/tickets/{}",
                ticket.slug
            ))
            .header("Authorization", format!("Token token={account_token}"))
            .send()
            .await?
            .json::<AdminTicketResponse>()
            .await?;

        info!("Ticket fetched! Printing now");

        if let Err(error) = checkin_output.checkin(&ticket.ticket) {
            error!("Error occured when printing ticket: {error}");
        }
    } else {
        info!("Ticket is not checked in. Checking in and reprinting ticket.");

        client
            .post(format!(
                "https://checkin.tito.io/checkin_lists/{checkin_slug}/checkins"
            ))
            .json(&NewCheckinBody::new(ticket.id))
            .send()
            .await?;

        info!("Checkin successfully created! The ticket will be printed if a run loop instance is also running. If not, run the command again.");
    }

    Ok(())
}

async fn run_loop(
    message: Message,
    checkin_output: &impl CheckinOutput,
) -> Result<(), HackNottsCheckinError> {
    let message_string = message.to_text()?;

    debug!("Received message from WebSocket");
    debug!("{message_string}");

    let checkin: WebhookCheckin = serde_json::from_str(message_string)?;

    info!("Processing checkin {}", checkin.id);

    if let Err(error) = checkin_output.checkin(&checkin) {
        error!("Error occured when printing ticket: {error}");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(); // what a line

    let args = Args::parse();

    let checkin_output = TypstCheckinOutput::new(args.template_path);

    if let (
        Some(recheck_id),
        Some(account_slug),
        Some(account_token),
        Some(checkin_slug),
        Some(event_slug),
    ) = (
        args.reference,
        args.account_slug,
        args.account_token,
        args.checkin_slug,
        args.event_slug,
    ) {
        // If the recheck arguments were specified, recheck and don't continue on to the runloop
        recheck(
            &recheck_id,
            &account_slug,
            &account_token,
            &checkin_slug,
            &event_slug,
            &checkin_output,
        )
        .await
        .expect("Failed to recheck!");
        return;
    }

    let url = "wss://wh2ws.hacksoc.net/websocket/checkin-created";

    info!("Connecting to {url}");
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    info!("WebSocket handshake has been successfully completed");

    while let Some(Ok(message)) = ws_stream.next().await {
        if let Err(e) = run_loop(message, &checkin_output).await {
            error!("Error occured when processing response: {e}");
        }
    }
}
