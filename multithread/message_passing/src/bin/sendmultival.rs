use std::thread;
use std::sync::mpsc::{self, channel};
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

	thread::spawn(move || {
	 	let val = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for v in val {
		 	tx.send(v).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("Got: {:?}", received);
	}

//	let received = rx.recv().unwrap();
//	println!("GotL {}", received);
}
