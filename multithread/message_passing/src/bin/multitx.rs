use std::thread;
use std::sync::mpsc::{self, channel, Sender};
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
	let tx1 = Sender::clone(&tx); 

	thread::spawn(move || {
	 	let val = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for v in val {
		 	tx1.send(v).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	thread::spawn(move || {
	 	let val = vec![
			String::from("more"),
			String::from("message"),
			String::from("for"),
			String::from("you"),
		];

		for v in val {
		 	tx.send(v).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("Got: {:?}", received);
	}
}
