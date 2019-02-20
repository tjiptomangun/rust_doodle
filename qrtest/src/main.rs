extern crate reqwest;
extern crate qrcode;
extern crate image;


use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use hyper::header::HeaderMap;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::CONTENT_LENGTH;
use qrcode::QrCode;
use image::Luma;
use image::ColorType;
use reqwest::Response;
use std::str;

struct FooBar<'a> {
	foo:String,
	burl: &'a str,
}


//https://stackoverflow.com/questions/50731636/how-do-i-encode-a-rust-piston-image-and-get-the-result-in-memory
fn main() { 
	//header! { (ContentType, "Content-Type") => [String] }
	//header! { (ContentLength, "Content-Length") => [usize] }

	if let Some(arg1) = env::args().nth(1){

		// let code = match QrCode::new(arg1.as_bytes()){
		// 	Ok(t) =>
		// 		t,
		// 	Err(err) =>
		// 		panic!("Cannot encode {}", err),
		// };

		let mut foo_bar = FooBar{
			foo: "foo".to_string(),
			burl: "",
		};

		let code = QrCode::new(arg1.as_bytes()).unwrap();

		let images = code.render::<Luma<u8>>().build();

		let mut buf:Vec<u8> = Vec::new();		
		
		let dims = images.dimensions();

		let mut headers = HeaderMap::new();		
		
		image::png::PNGEncoder::new(buf.by_ref())
		 	.encode(
				 &images.into_raw(), 
				 dims.0, 
				 dims.1, 
				 ColorType::Gray(8),
			).expect("Error on encoding to png");
		
		let len = buf.len();
		let cl = format!("{}", len);

		headers.insert(CONTENT_TYPE, "image/png".parse().unwrap());
		headers.insert("Slug", "photo.jpg".parse().unwrap());
		headers.insert(CONTENT_LENGTH, cl.parse().unwrap());

		let client = ::reqwest::Client::new();

		let mut response:Response = match client.post("http://localhost:8888/image")
			.body(buf)
			.headers(headers)
			.send() {
				Ok(t) => t,
				Err(error) =>{
					panic!("There are problem: {:?}", error)
				},
			};

		println!("Headers:\n{:?}", response.headers());

		let hloc = response.headers().get("Location").unwrap();

		foo_bar.burl = str::from_utf8(hloc.as_bytes()).unwrap();

		// println!("hloc  {:?}", hloc);
		println!("foo_bar.burl  {:?}", foo_bar.burl);


	}
	else {
		println!("Please supply string to encode");
	}
	
}
