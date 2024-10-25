use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Checkin {
    /// The date the Checkin was created
    pub created_at: DateTime<Utc>,

    /// The date the checkin was deleted
    pub deleted_at: Option<DateTime<Utc>>,

    /// The identifier of the Checkin
    pub id: u64,

    /// The id of the ticket that was checked-in
    pub ticket_id: u64,

    /// The reference of the ticket that was checked-in
    pub ticket_reference: Option<String>,

    /// The universally unique identifier of the Checkin
    pub uuid: Uuid,

    /// The date the Checkin was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticket {
    pub created_at: DateTime<Utc>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: u64,
    pub last_name: Option<String>,
    pub reference: Option<String>,
    pub registration_reference: String,
    pub release_title: String,
    pub slug: String,
    pub updated_at: DateTime<Utc>,
}

/// The POST body for creating a new checkin from a ticket ID
#[derive(Serialize, Deserialize, Debug)]
pub struct NewCheckinBody {
    checkin: NewCheckinBodyCheckin,
}

impl NewCheckinBody {
    pub fn new(ticket_id: u64) -> Self {
        Self {
            checkin: NewCheckinBodyCheckin { ticket_id },
        }
    }
}

/// For some reason the Tito API makes you put the ticket ID in a little inner class like this:
/// `{"checkin":{"ticket_id":111111}}`, so this struct exists to accommodate that
#[derive(Serialize, Deserialize, Debug)]
struct NewCheckinBodyCheckin {
    pub ticket_id: u64,
}

/// A checkin received from the webhook. Not to be confused with [Checkin], which is returned by the
/// check-in API.
#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookCheckin {
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub company_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub release_title: String,
    pub email: String,
    pub reference: String,
    pub registration_reference: String,
    pub checked_in: bool,
    pub checked_in_at: DateTime<Utc>,
    pub checkin_list: CheckinList,
    pub checkin_uuid: String,
    pub custom: Option<serde_json::Value>,
    pub event: Event,
    pub answers: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinList {
    pub slug: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub slug: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub response: String,
}

/// The admin API actually returns way more than just responses, so much so that I really don't want
/// to type out the full definition. This definition just includes enough to implement
/// [CheckinPrintable]
#[derive(Serialize, Deserialize, Debug)]
pub struct AdminTicket {
    pub name: String,
    pub reference: String,
    pub release_title: String,
    pub responses: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminTicketResponse {
    pub ticket: AdminTicket,
}

impl fmt::Display for WebhookCheckin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Checkin ID: {}\n", self.id)?;
        write!(f, "Slug: {}\n", self.slug)?;
        write!(f, "Name: {} {}\n", self.first_name, self.last_name)?;
        write!(f, "Company: {}\n", self.company_name)?;
        write!(f, "Release Title: {}\n", self.release_title)?;
        write!(f, "Email: {}\n", self.email)?;
        write!(f, "Reference: {}\n", self.reference)?;
        write!(
            f,
            "Registration Reference: {}\n",
            self.registration_reference
        )?;
        write!(f, "Checked In: {}\n", self.checked_in)?;
        write!(f, "Checked In At: {}\n", self.checked_in_at)?;
        write!(f, "Checkin List: {}\n", self.checkin_list)?;
        write!(f, "Checkin UUID: {}\n", self.checkin_uuid)?;
        write!(f, "Custom: {:?}\n", self.custom)?;
        write!(f, "Event: {}\n", self.event)?;
        write!(f, "Answers: {:?}\n", self.answers)
    }
}

impl Display for CheckinList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.slug)
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.slug)
    }
}
