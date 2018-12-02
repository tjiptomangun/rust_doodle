extern crate android_glue;

fn main() {
    let i = 0;
    android_glue::write_log("main() has been called");
    loop {}
}
