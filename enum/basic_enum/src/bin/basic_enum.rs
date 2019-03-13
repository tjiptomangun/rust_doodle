enum IpAddr{
	V4(u8, u8, u8, u8),
	V6(String),
}

impl IpAddr {
	fn call(&self){
	}
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));
	loopback.call();
	home.call();
}
