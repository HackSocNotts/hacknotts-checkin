use std::error::Error;

use log::warn;

use crate::tito_types::{AdminTicket, WebhookCheckin};

pub trait CheckinOutput {
    type Error: Error;

    fn checkin(&self, checkin: &impl CheckinPrintable) -> Result<(), Self::Error>;
}

pub trait CheckinPrintable {
    fn name(&self) -> String;
    fn reference(&self) -> String;
    fn discord(&self) -> String;
    fn pizza(&self) -> String;
    fn release_title(&self) -> String;
}

impl CheckinPrintable for WebhookCheckin {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn reference(&self) -> String {
        self.reference.clone()
    }

    fn discord(&self) -> String {
        let question = self
            .answers
            .iter()
            .find(|answer| answer.question == "What is your Discord username?");

        match question {
            Some(answer) => answer.response.clone(),
            None => {
                warn!("Failed to find Discord username! The ticket was: {self:?}");
                "???".to_string()
            }
        }
    }

    fn pizza(&self) -> String {
        let question = self
            .answers
            .iter()
            .find(|answer| answer.question == "Pizza Choice!");

        match question {
            Some(answer) => answer.response.clone(),
            None => {
                warn!("Failed to find pizza choice! The ticket was: {self:?}");
                "???".to_string()
            }
        }
    }

    fn release_title(&self) -> String {
        self.release_title.clone()
    }
}

impl CheckinPrintable for AdminTicket {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn reference(&self) -> String {
        self.reference.clone()
    }

    fn discord(&self) -> String {
        let question = self.responses.get("what-is-your-discord-username");

        match question {
            Some(answer) => answer.clone(),
            None => {
                warn!("Failed to find Discord username! The ticket was: {self:?}");
                "???".to_string()
            }
        }
    }

    fn pizza(&self) -> String {
        let question = self.responses.get("pizza-choice");

        match question {
            Some(answer) => answer.clone(),
            None => {
                warn!("Failed to find pizza choice! The ticket was: {self:?}");
                "???".to_string()
            }
        }
    }

    fn release_title(&self) -> String {
        self.release_title.clone()
    }
}
