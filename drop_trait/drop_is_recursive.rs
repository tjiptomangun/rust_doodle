struct Inner;
struct Outer(Inner);

impl Drop for Inner {
    fn drop(&mut self) {
        println!("Drop Inner!");
    }    
}

impl Drop for Outer {
    fn drop(&mut self) {
        println!("Drop Outer!");
    }
}

fn main () {
    let _x = Outer(Inner);
}
