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

fn iteration_invalidation() {
	let mut v = vec![1, 2, 3];

	for i in &v {
		println!("{}", i);
		//v.push(4); //cannot borrow v as mutable as it is also borrowed as immutable
	}
}

fn main () {
	unscoped();
	scoped();
}
