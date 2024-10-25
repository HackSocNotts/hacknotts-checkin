use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::checkin_output::{CheckinOutput, CheckinPrintable};
use log::{debug, info, warn};
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
pub struct TypstCheckinOutput {
    /// The path to the template file. For example, `template/main.typ`
    template_path: PathBuf,
}

impl TypstCheckinOutput {
    pub fn new(template_path: PathBuf) -> Self {
        Self { template_path }
    }
}

impl CheckinOutput for TypstCheckinOutput {
    type Error = TypstCheckinOutputError;

    fn checkin(&self, checkin: &impl CheckinPrintable) -> Result<(), Self::Error> {
        let template_parent = self.template_path.parent().unwrap_or(Path::new("/"));

        // For some reason [fs::read_to_string] uses a boxed error, so we have this ugly map
        let mut template = fs::read_to_string(&self.template_path)
            .map_err(|e| TypstCheckinOutputError::TemplateReadError(Box::new(e)))?;

        // You could totally break through this sanitisation with a good enough input (just having
        // \" would break it, off the top of my head). If a hacker does that, they deserve their
        // broken ticket.
        template = template.replace("{{reference}}", &checkin.reference().replace('"', "\\\""));
        template = template.replace("{{name}}", &checkin.name().replace('"', "\\\""));
        template = template.replace("{{discord}}", &checkin.discord().replace('"', "\\\""));
        template = template.replace("{{pizza}}", &checkin.pizza().replace('"', "\\\""));

        let mut typst_command = Command::new("typst")
            .current_dir(template_parent)
            .arg("compile")
            .arg("-") // Take the modified template from stdin
            .arg("-") // Output to stdout
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        typst_command
            .stdin
            .as_mut()
            .ok_or(TypstCheckinOutputError::NoStdin)?
            .write_all(template.as_bytes())
            .map_err(|_| TypstCheckinOutputError::NoStdin)?;

        let typst_output = typst_command.wait_with_output()?;

        debug!("Typst exited with status {}", typst_output.status);

        if !typst_output.stderr.is_empty() {
            let typst_stderr_string = String::from_utf8_lossy(&typst_output.stderr);

            warn!("Typst stderr was non-empty:\n{typst_stderr_string}");
        }

        let mut lp_command = Command::new("lp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        lp_command
            .stdin
            .as_mut()
            .ok_or(TypstCheckinOutputError::NoStdin)?
            .write_all(&typst_output.stdout)?;

        let lp_output = lp_command.wait_with_output()?;

        debug!("lp exited with status {}", lp_output.status);

        info!("Checkin {} printed", checkin.reference());

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum TypstCheckinOutputError {
    #[error("Failed to get stdin")]
    NoStdin,

    #[error("Failed to get stdout")]
    NoStdout,

    #[error("Failed to read the template: {0:?}")]
    TemplateReadError(Box<dyn std::error::Error>),

    #[error("Failed to read/write to a command: {0}")]
    CommandIOError(#[from] io::Error),

    #[error("Failed to find variable {0} in template")]
    VariableNotFound(String),
}
