#[repr(C)]
struct RustObject {
    a: i32,
}

extern "C" fn callback(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        (*target).a = a;
    }
}

#[link(name="crustcaller")]
extern {
    fn register_callback(target: *mut RustObject,
                         cb: extern fn(*mut RustObject, i32))-> i32;
    fn trigger_callback();
}

fn main() {
    // Create the object that will be referenced in callback
    let mut rust_object = Box::new(RustObject{a: 5});
    println!("pre set {}", &rust_object.a);
    unsafe {
        register_callback(&mut *rust_object, callback);
        trigger_callback();
    }
    println!("post set {}", &rust_object.a);

}
