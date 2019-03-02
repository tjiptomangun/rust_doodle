//This will not work 
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
			let counter = Arc::clone(&counter);
			let handle = thread::spawn(move || {//std::rc::Rc<std::sync::Mutex<i32>> cannot be sent between threads safely
												//the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>

					let mut num = counter.lock().unwrap();
					*num += 1;
				}
			);
			handles.push(handle);
	}

	for handle in handles{
		handle.join().unwrap();
	}
	
	println!("Result: {}", *counter.lock().unwrap());
}
