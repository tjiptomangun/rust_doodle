use std::thread;
use std::sync::mpsc::{self, channel};

fn main() {
    let (tx, rx) = channel();

	thread::spawn(move || {
	 	let val = String::from("hi");
	 	tx.send(val).unwrap();
	});
}
