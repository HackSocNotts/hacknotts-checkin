use std::error::Error;

use crate::tito_types::WebhookCheckin;

pub trait CheckinOutput {
    type Error: Error;

    fn checkin(checkin: &WebhookCheckin) -> Result<(), Self::Error>;
}
