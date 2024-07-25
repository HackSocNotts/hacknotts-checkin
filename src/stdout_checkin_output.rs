use crate::checkin_output::{CheckinOutput, CheckinPrintable};

pub struct StdOutCheckinOutput;

impl CheckinOutput for StdOutCheckinOutput {
    fn checkin(checkin: &impl CheckinPrintable) {
        println!("HackNotts 24");
        println!("{}", checkin.name());
        println!("{}", checkin.reference());
    }
}
