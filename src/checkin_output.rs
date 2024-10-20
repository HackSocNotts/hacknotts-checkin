use std::error::Error;

use crate::tito_types::{Ticket, WebhookCheckin};

pub trait CheckinOutput {
    type Error: Error;

    fn checkin(checkin: &impl CheckinPrintable) -> Result<(), Self::Error>;
}

pub trait CheckinPrintable {
    fn name(&self) -> String;
    fn reference(&self) -> String;
    fn discord(&self) -> String;
    fn pizza(&self) -> String;
}

impl CheckinPrintable for Ticket {
    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn reference(&self) -> String {
        self.reference.clone()
    }

    fn discord(&self) -> String {
        unimplemented!()
    }

    fn pizza(&self) -> String {
        unimplemented!()
    }
}

impl CheckinPrintable for WebhookCheckin {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn reference(&self) -> String {
        self.reference.clone()
    }

    fn discord(&self) -> String {
        unimplemented!()
    }

    fn pizza(&self) -> String {
        unimplemented!()
    }
}
