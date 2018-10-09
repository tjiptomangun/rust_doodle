extern crate android_glue;
use std::{thread, time};

fn main() {
    let i = 0;
    let ten_second= time::Duration::from_millis(10000);
    let seven_second= time::Duration::from_millis(7000);

    android_glue::write_log("main() has been called");

    let h1 = thread::spawn(move || {
        loop {
            thread::sleep(seven_second);
            android_glue::write_log("child() wake up");
        }
    });
    loop { 
        thread::sleep(ten_second);
        android_glue::write_log("main() wake up");
    }

    h1.join().unwrap();
}
