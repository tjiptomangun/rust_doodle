extern crate tokio_core;
extern crate twilio_async;

use std::{env, error::Error};
use tokio_core::reactor::Core;
use twilio_async::{
    twiml::{Dial, Response, Twiml},
    MsgResp, Twilio, TwilioRequest,
};

fn main() -> Result<(), Box<Error>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    let mut core = Core::new()?;    

    //sending a message 
    try_msg(&mut core, twilio)?;
    Ok(())
}

fn try_msg(core: &mut Core, twilio: Twilio) -> Result<(), Box<Error>> {
        let num = env::var("OUTBOUND_NUM")?;

        let (_, resp) = core.run(twilio.send_msg("+18572148664",&num, "Hello World").run())?;

        println!("{:?}", resp);

        //sending with media
        let (_, resp) = core.run(twilio
                .send_msg("+18572148664", &num, "foo")
                .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
                .run(),
            )?;

        //get individual message
        
        if let Some(json) = resp {
            let MsgResp {sid, .. } = json;
            let (_, resp) = core.run(twilio.msg("MMec83347e541440f389e24377dd901af7").run())?;
            println!("{:?}", resp);
        }

        Ok(())
}


