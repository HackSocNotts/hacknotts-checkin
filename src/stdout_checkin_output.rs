use crate::{
    checkin_output::CheckinOutput, error::HackNottsCheckinError, tito_types::WebhookCheckin,
};

pub struct StdOutCheckinOutput;

impl CheckinOutput for StdOutCheckinOutput {
    type Error = HackNottsCheckinError;

    fn checkin(checkin: &WebhookCheckin) -> Result<(), Self::Error> {
        println!("HackNotts 24");
        println!("{}", checkin.name);
        println!("{}", checkin.reference);

        Ok(())
    }
}
