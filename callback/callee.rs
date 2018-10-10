extern fn callback(a: i32){
    println!("I'm called from C with value{0}", a);
}

#[link(name="caller")]
extern {
    fn register_callback(cb: extern fn(i32)) -> i32;
    fn trigger_callback();
}

fn main() {
    unsafe {
        register_callback(callback);
        trigger_callback();
    }
}
