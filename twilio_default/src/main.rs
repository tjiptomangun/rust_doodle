use twilio_async::{
    twiml::{Dial, Response, Twiml},
    MsgResp, Twilio, TwilioRequest,
}; 

use std::{env, error::Error};

extern crate futures; // 0.1.24

use futures::{future, Future};



fn main() -> Result<(), Box<Error>>{
   let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?).unwrap(); 
   try_msg(twilio)?;
   Ok(())
}

fn try_msg(twilio: Twilio) -> Result<(), Box<Error>> {
    let num = env::var("OUTBOUND_NUM")?;
    // sending a message
    let resp = twilio.send_msg("6281398127441", &num, "Hello World").run().wait();

	println!("{:?}", resp);
    Ok(())
}
