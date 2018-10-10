struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}

fn test_cleanup() {
    let c = CustomSmartPointer{data: String::from("my stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
}

fn main() {
    println!("calling test cleanup");
    test_cleanup();
    println!("test cleanup called");
}
