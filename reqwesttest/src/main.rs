extern crate reqwest;

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use hyper::header::HeaderMap;


use reqwest::header::CONTENT_TYPE;
use reqwest::header::CONTENT_LENGTH;


use reqwest::Response;
use hyper::header::AsHeaderName;

fn main() { 
	//header! { (ContentType, "Content-Type") => [String] }
	//header! { (ContentLength, "Content-Length") => [usize] }

	if let Some(arg1) = env::args().nth(1){
		let metadata = fs::metadata(&arg1).unwrap();
		let mut file = fs::File::open(&arg1).unwrap();
		let mut buffer:Vec<u8> = Vec::new();

		buffer.clear(); 
		file.read_to_end(&mut buffer).expect("cannot read buffer into file"); 
		println!("buff length {:?}", buffer.len()); 

		let client = ::reqwest::Client::new();

		let mut headers = HeaderMap::new();

		let cl = format!("{}", metadata.len());

		headers.insert(CONTENT_TYPE, "image/jpeg".parse().unwrap());
		headers.insert(CONTENT_LENGTH, cl.parse().unwrap());
		headers.insert("Slug", "photo.jpg".parse().unwrap());

		let mut response:Response = match client.post("http://localhost:8888/image")
			.body(buffer)
			.headers(headers)
			.send() {
				Ok(t) => t,
				Err(error) =>{
					panic!("There are problem: {:?}", error)
				},
			};

		let text = response.text().unwrap();
		let mut response_body = String::new();
		//https://docs.rs/reqwest/0.8.6/reqwest/struct.Response.html
		//file:///home/hanky/rust/reqwesttest/target/doc/hyper/header/trait.AsHeaderName.html
		//https://github.com/seanmonstar/reqwest/blob/master/examples/simple.rs
		println!("Headers:\n{:?}", response.headers());

		let hloc = response.headers().get("Location").unwrap();		
		println!("hloc  {:?}", hloc);

		
	}
	else {
		println!("Need filename");
	}
	
}
