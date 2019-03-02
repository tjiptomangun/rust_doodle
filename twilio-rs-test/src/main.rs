use twilio::{self, OutboundMessage};

fn main() {
	let client = twilio::Client::new("xxxxxxxxxxxxxxxxxxxx", "xxxxxxxxxxxxxxxxxxxxxxx");
	match client.send_message(OutboundMessage::new("+xxxxxxxxxxxxxxxx", "+xxxxxxxxxxxxxx", "Hello World!")){
		Err(e) => println!("error {:?}", e),
		Ok(m) => println!("ok {:?}", m),
	}
	
}
