use crate::{checkin_output::CheckinOutput, error::HackNottsCheckinError, tito_types::Checkin};

pub struct StdOutCheckinOutput;

impl CheckinOutput for StdOutCheckinOutput {
    type Error = HackNottsCheckinError;

    fn checkin(checkin: &Checkin) -> Result<(), Self::Error> {
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

        println!("{:^40}", checkin.name);
        println!("{:^40}", checkin.reference);

        Ok(())
    }
}
