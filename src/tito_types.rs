use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Checkin {
    id: u64,
    slug: String,
    name: String,
    company_name: String,
    first_name: String,
    last_name: String,
    release_title: String,
    email: String,
    reference: String,
    registration_reference: String,
    checked_in: bool,
    checked_in_at: DateTime<Utc>,
    checkin_list: CheckinList,
    checkin_uuid: String,
    custom: Option<serde_json::Value>,
    event: Event,
    answers: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinList {
    slug: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    slug: String,
    title: String,
}

impl fmt::Display for Checkin {
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
