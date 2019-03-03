struct UndebugRectangle {
	width: u32,
	height: u32,
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	/*
	fn arena() -> u32 {
		width * height
	}
	*/

	//An associated function, called with :: and does not need self
	fn square(size: u32) -> Rectangle {
		Rectangle{width: size, height: size}
	}
}

fn main() {
	let rect1 = UndebugRectangle{width:30, height: 50};
	let rect2 = Rectangle{width:30, height: 50};

	//println!("rect1 is {:?}", rect1); cannot be formatted
	println!("rect2 is {:?}", rect2);
	println!("rect2 is {:#?}", rect2);

	println!("The area of the rectangle is {} square pixels", rect2.area());

	let 
}
