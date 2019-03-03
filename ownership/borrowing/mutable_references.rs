fn unscoped() {
	let mut x = 5;
	let y = &mut x;
	*y += 1;

	//println!("{}", x);	//cannot borrow x as immutable because it is also
						//borrowed as mutable
						//}//main mutable borrow ends here
					
}//main


fn scoped () { 
	let mut x = 5;
	{
		let y = &mut x;
		*y += 1;
	}
	println!("{}", x);
}

fn iterator_invalidation() {
	let mut v = vec![1, 2, 3];

	for i in &v {
		println!("{}", i);
		//v.push(4); //cannot borrow v as mutable as it is also borrowed as immutable

		//why this is not allowed is simply it will cause segmentation fault 
		//and infinite loop
	}
}

fn use_after_free() {
	let y : &i32;
	{
		let x = 5;
	//	y = &x;
	}//x dropped while still borrowed (by y)
	//y is still using x whilst x has already freed because the scope x declared
	//in has already end

	//println!("{}", y);
}

fn use_after_free2() {
	let y : &i32;
	
	let x = 5;
	//y = &x;//borrowed value does not live long enough

	//the reason is in the error message note
	//note: values in a scope are dropped in the opposite order they are created

	//which means on leaving scope (here it is a function),  x will be deleted 
	//first then y
	//but y is still holding reference to x
	
}

fn main () {
	unscoped();
	scoped();
	iterator_invalidation();
	use_after_free();
	use_after_free2();
}
