use snowflake_multi_threaded::SnowFlakeId;
use std::str;

struct FooBar<'a> {
	foo:String,
	burl: &'a str,
}

fn main() {
		let mut foo_bar = FooBar{
			foo: "foo".to_string(),
			burl: "",
		};	
	let workerId: i64 = 1;
	let datacenterId: i64 = 1;
	let mut id_gen = SnowFlakeId::new(workerId, datacenterId);
	let r = match id_gen.generate_id() {
		Ok(t) =>
			t,
		Err(err) =>
			panic!("Error {}", err),
	};
	let ret = format!("{:X}", r);

	foo_bar.burl = str::from_utf8(ret.as_bytes()).unwrap();

	println!("result {}", foo_bar.burl);
	
}
