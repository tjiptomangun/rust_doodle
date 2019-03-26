use bytes::{BytesMut, BufMut, Bytes};
use std::str;

fn main() {
	let mut buf = BytesMut::with_capacity(1024);

	buf.put(&b"hello world"[..]);
	buf.put_u16_be(1234);

	let a = buf.take();
	assert_eq!(a, b"hello world\x04\xD2"[..]);

	let a = Bytes::from(&b"hello"[..]);
	let b = Bytes::from(&b" world"[..]);
	let c = Bytes::from(&b" dennis"[..]);
	let d = Bytes::from(&b" m"[..]);
	let e = Bytes::from(&b" ritchie"[..]);

	let j:Vec<Bytes> = vec![a, b, c, d, e];

	let res = j.iter().fold(BytesMut::with_capacity(0), |mut acc, bytes|{
		acc.extend_from_slice(bytes);
		acc
	});

	let rest_str = str::from_utf8(&res);

	println!("result string : {:?}", rest_str);

}
