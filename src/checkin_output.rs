use crate::tito_types::{Ticket, WebhookCheckin};

pub trait CheckinOutput {
    fn checkin(checkin: &impl CheckinPrintable);
}

pub trait CheckinPrintable {
    fn name(&self) -> String;
    fn reference(&self) -> String;
}

impl CheckinPrintable for Ticket {
    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn reference(&self) -> String {
        self.reference.clone()
    }
}

impl CheckinPrintable for WebhookCheckin {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn reference(&self) -> String {
        self.reference.clone()
    }
}
