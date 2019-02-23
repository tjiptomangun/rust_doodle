fn main() {
	let a: i32 = 1;
	let _b: i32 = a;
	let _c: i32 = a;	
	let _d: i32 = a;

	let aa: bool = true;
	let _bb: bool = aa;
	let _cc: bool = aa;
	let _dd: bool = aa;

	let aaa: &str = "Hello World";
	let _bbb = aaa;
	let _ccc = aaa;
	let _ddd = aaa;

	let sss = [1, 2, 3,4, 5];
	let _ttt = sss;
	let _uuu = sss;
	let _vvv = sss;


	let v1:Vec<i32> = vec!(1,2,3);
	let _v2:Vec<i32> = v1;
	//let v3:Vec<i32> = v1; //used after moved, no copy trait 

	let _www = &sss[1..2];
	let _yyy = &sss[1..2];
	let _zzz = &sss[1..2];

	let x = (1, "Hello World");
	let _x = x;
	let _y = x;
	let _r = x;

	let _x0 = x.0;
	let _x1 = x.1;

	fn foo(x: i32) -> i32 {x};
	let _foox: fn(i32) -> i32 = foo; 
	let _fooy: fn(i32) -> i32 = foo; 
	let _fooz: fn(i32) -> i32 = foo; 

	let str0: String = "Hello World".to_string();
	let _str1: String = str0;
	//let str2: String = str0; //used after moved, no copy trait

	let _y = double(a);
	let _y1 = double(a);
	let _y2 = double(a);

	let _z = change_truth(aa);
	let _z1 = change_truth(aa);
	let _z2 = change_truth(aa);

	let vecs: Vec<i32> = vec!(1, 2, 3);
	let _vect: Vec<i32> = dupvec(vecs);
	//let _vecu: Vec<i32> = dupvec(vecs);//used after moved, no copy trait
}

fn double(x: i32) -> i32 {
	x * 2
}

fn change_truth(x: bool) -> bool{
	!x
}

fn dupvec(iv: Vec<i32>) -> Vec<i32> {
	iv
}
