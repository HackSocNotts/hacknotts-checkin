use crate::checkin_output::{CheckinOutput, CheckinPrintable};
use thiserror::Error;

/// Prints a ticket with CUPS from a Typst template.
///
/// To render the Typst template, the Typst CLI is used. I'm pretty sure you *can* invoke Typst
/// directly from Rust, but the API looks like it's very low-level, and would be more work than it's
/// worth. The API documentation for Typst can be found at https://docs.rs/typst/latest/typst/ if
/// you're interested.
///
/// To insert user ticket data into the template, a simple variable format is used. The variables
/// are as follows:
///
/// * `{{reference}}` - The Tito ticket reference. For example, `HKUW-1`. Represented in
/// [CheckinPrintable::reference]
/// * `{{name}}` - The attendee's name. For example, `James Harvey`. Represented in
/// [CheckinPrintable::name]
/// * `{{discord}}` - The attendee's Discord username. For example, `jmshrv`. Represented in
/// [CheckinPrintable::discord]
/// * `{{pizza}}` - The attendee's pizza choice. For example, `Pepperoni`. Represented in
/// [CheckinPrintable::pizza]
///
/// These fields are filled in simply by doing a find/replace on the variables. If a variable is not
/// found, [TypstCheckinOutputError::VariableNotFound] will be thrown with the missing variable.
/// Multiple replacements are permitted.
///
/// The `typst` command is run in the directory of the template, so that any images/fonts get picked
/// up by Typst.
///
/// The ticket is printed to the default printer with the `lp` command.
pub struct TypstCheckinOutput {}

impl CheckinOutput for TypstCheckinOutput {
    type Error = TypstCheckinOutputError;

    fn checkin(checkin: &impl CheckinPrintable) -> Result<(), Self::Error> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum TypstCheckinOutputError {
    #[error("Failed to find variable {0} in template")]
    VariableNotFound(String),
}
