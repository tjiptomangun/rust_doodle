/*
 * https://doc.rust-lang.org/book/first-edition/raw-pointers.html
 * Rust has a number of different smart pointer types in its standard library, 
 * but there are two types that are extra-special. Much of Rust’s safety comes from compile-time checks, 
 * but raw pointers don’t have such guarantees, and are unsafe to use.  

 
 *  *const T and *mut T are called ‘raw pointers’ in Rust
*/

fn main() {
    let x = 5;
    let raw = &x as *const i32;

    let mut y = 10;
    let raw_mit = &mut y as *mut i32;

    /* Dereferencing won't work 
     * println!("raw points at {}", *raw); 
     * dereferenceo of raw pointer requires unsafe function
     */

    let points_at = unsafe{ *raw};
    println!("raw points at {}", points_at); 

    //conversion from &T to *const T or *mut T is safe
    //but the reverse is not

    //Explicit conversion
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    //Implicit conversion
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }




}
