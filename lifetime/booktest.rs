fn main () {
	let r;

	{
		let x = 5;
		//r = &x;//borrowed value doesn't live long enough
		r = x;
	}

	println!("r : {}", r);
}
