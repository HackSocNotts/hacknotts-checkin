use crate::{
    checkin_output::{CheckinOutput, CheckinPrintable},
    error::HackNottsCheckinError,
};

/// Prints checkins to stdout, only really made for testing
pub struct StdOutCheckinOutput;

impl CheckinOutput for StdOutCheckinOutput {
    // This checkin output never actually errors, but I don't think you can specify `()` since it
    // needs to conform to `Error`.
    type Error = HackNottsCheckinError;

    fn checkin(&self, checkin: &impl CheckinPrintable) -> Result<(), Self::Error> {
        let hacknotts_logo = " __   __   __     
|  |_|  |_|  |    
|            |    
'-----,      |    
      |      |    
   /  |  \\   |    
   \\  |  /   |    
      |      |    
      '------' ";
        println!("{:^40}", "HackNotts 24");

        for line in hacknotts_logo.lines() {
            println!("{:^40}", line);
        }

        println!("{:^40}", checkin.name());
        println!("{:^40}", checkin.reference());

        Ok(())
    }
}
