unsafe fn test_transmute(a: &str) -> &str{
    let s: &str= std::mem::transmute(a);
    s
}
fn main () {
    let x = String::from("terong");
    let y = x.as_str();
    let z:&str;
    unsafe {z = test_transmute(&y);}
    //let (a: *str, b: usize)= std::mem::transmute(x);
    println!("z {}", z);
}
