fn print_me_string(msg: String) {
	println!("the message is {}", msg);
}

fn print_me_str(msg: &str) {
	println!("the message is {}", msg);
}

fn main() {
	let msg = "hello world!";
	// print_me_string(msg);  |    expected struct `std::string::String`, found &str
    //                       |    help: try using a conversion method: `msg.to_string()`

	//So a string literal is of type &str and does not appear compatible with the type String
    //msg.to_string() using clone
	print_me_str(msg);

	let owned_string = "hello world".to_string();
	print_me_str(&owned_string);

}
