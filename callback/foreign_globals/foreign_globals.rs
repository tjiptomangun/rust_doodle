extern crate libc;

#[link(name = "readline")]
extern "C" {
    static rl_readline_version: libc::c_int;
}

fn main() {
    println!("You have readline version {} installed", unsafe {
        rl_readline_version as i32
    });
}
