fn main() {
	let rect1 = (30, 50);
	println!("The area of the rectangle is {} square pixels", area(rect1));
	println!("rect1 {} x {}", rect1.0, rect1.1);
}

fn area(dimensions:(u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}
