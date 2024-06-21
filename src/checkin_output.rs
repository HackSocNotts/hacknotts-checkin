use std::error::Error;

use crate::tito_types::Checkin;

pub trait CheckinOutput {
    type Error: Error;

    fn checkin(checkin: &Checkin) -> Result<(), Self::Error>;
}
