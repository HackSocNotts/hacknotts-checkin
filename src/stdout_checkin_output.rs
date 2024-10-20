use crate::checkin_output::{CheckinOutput, CheckinPrintable};

pub struct StdOutCheckinOutput;

impl CheckinOutput for StdOutCheckinOutput {
    fn checkin(checkin: &impl CheckinPrintable) {
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
    }
}
