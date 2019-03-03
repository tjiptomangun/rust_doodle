struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rect1 = (30, 50);
	println!("The area of the rect1 is {} square pixels", area(rect1));
	println!("rect1 {} x {}", rect1.0, rect1.1);//using tuple, value is not moved
												//because its member are all in stack

	let rect2 = Rectangle {width:30, height: 50};
	println!("The area of the rect2 is {} square pixels", rectareptr(&rect2));
	println!("rect2 {} x {}", rect2.width, rect2.height); 
	let rect4 = &rect2;
	println!("rect2 {} x {}", rect2.width, rect2.height); // borrow and we can still use

	let rect3 = Rectangle {width:30, height: 50};
	println!("The area of the rect2 is {} square pixels", rectares(rect3));
	//println!("rect3 {} x {}", rect3.width, rect3.height); using struct value is moved
}

fn area(dimensions:(u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn rectareptr(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

fn rectares(rectangle: Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
