fn main() {
	let s = String::from("hello");

	takes_ownership(s);

	let x = 5;

	makes_copy(x);

	println!("x: {}", x);
	//println!("s: {}", s); -- value moved, s is heap variable and already moved
	//						-- and freed in fn takes_ownership by drop
}

fn takes_ownership(some_string: String){//some string comes into scope
	println!("{}", some_string);
}// Here, some_string goes out of scoped and `drop` is called. The
 // backing memory is freed. 

fn makes_copy(some_integer: i32) {//some_integer comes into scope
	println!("{}", some_integer);
}// Here, some_integer goes out of scope. Nothing special happens
