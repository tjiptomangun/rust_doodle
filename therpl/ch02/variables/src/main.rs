const MAX_POINTS: u32 = 100_000;

fn main() {
	let mut x = 5;
	println!("The value of x is : {}", x);

	x = 6;	
	println!("The value of x is : {}", x);

	let x = x + 1;

	let x = x * 2;
	println!("The value of x is : {}", x);

	let spaces = "   ";
	println!("The value of spaces is : {}", spaces);

	let spaces = spaces.len();
	println!("The value of spaces is : {}", spaces); 

	let tup: (i32, f64, u8) = (500, 6.1, 1);
	println!("The value of tup is : {}", tup); 
}

