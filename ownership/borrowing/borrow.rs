fn main () {
	let a = "Hello World".to_string();

	{
		let _a = a;
	}

	//let _a1 = a;//used after moved
	//let _a2 = _a;//b not found in this scope

	let a = "Hello World".to_string();

	{
		let _a2 = &a;
	}

	let _a3 = &a;
	let _a4 = &a;
	let _a5 = &a;

	let v1 = vec!(1, 2, 3);	
	let v2 = vec!(1, 2, 3);	

	let _ff1 = foofekter(v1, v2);
	//let ff2 = foofekter(v1, v2);//value moved here

	let vv1 = vec!(1, 2, 3);	
	let vv2 = vec!(1, 2, 3);	

	let _fg1 = foofighter(&vv1, &vv2);
	let _fg2 = foofighter(&vv1, &vv2);

}

fn foofekter(v1: Vec<i32>, _v2: Vec<i32>) -> Vec<i32> {
	v1
}

fn foofighter(v1: &Vec<i32>, _v2: &Vec<i32>) -> Vec<i32> {
	v1.to_vec()
}
