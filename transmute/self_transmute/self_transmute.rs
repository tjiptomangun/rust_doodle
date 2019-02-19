extern crate libc;
use libc::{c_int, c_uchar};
struct Terong{
    val: [i32;4],
}

trait Transmuter {
    fn id(&self) ->*mut [c_uchar; 16];
}

impl Transmuter for Terong {
    fn id(&self) ->*mut [c_uchar; 16] { 
        unsafe{::std::mem::transmute(self)}
    }
}

fn main() {
    let j = Terong{val:[1, 2, 3, 4]};
    let h = unsafe{&j.id()};
    println!("h = {} ", &h[0]);
}
