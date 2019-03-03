fn main() {
    let mut s = String::from("hello");
	s.push_str(", world!");
	println!("{}", s);

	let x = 5;
	let y = x;

	println!("x: {}", x);

	let mut y = 5;
	{
		let z = 7;
		y = z + 1;
	}
	println!("y: {}", y);

	let mut s = String::from("terong");
	{
		let b = String::from(" goreng");
		s = format!("{} {}", s, b);
	}
	println!("s: {}", s);

	let mut y = s;
	//let mut z = s; value moved
}
