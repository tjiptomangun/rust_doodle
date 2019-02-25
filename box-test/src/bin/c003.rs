enum List{// recursive type has infinite size
	Cons(i32, Box<List>), 
	Nil,
}

use List::{Cons, Nil};

fn main() {
	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
