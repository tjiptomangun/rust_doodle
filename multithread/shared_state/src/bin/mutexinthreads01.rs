//This will not work
use std::sync::Mutex;
use std::thread;

fn main() {
	let counter = Mutex::new(0);
	let mut handles = vec![];

	for _ in 0..10 {
			let handle = thread::spawn(move || {//value moved into closure here, in previous iteration of loop 
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
