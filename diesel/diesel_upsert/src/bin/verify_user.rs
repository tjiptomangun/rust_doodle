extern crate diesel_upsert;
extern crate diesel;

use self::diesel_upsert::*;
use std::io::stdin;

fn main() {
	let connection = establish_connection();

	println!("name to verify: ");
	let mut name = String::new();
	stdin().read_line(&mut name).unwrap();
	let name = &name[..(name.len() - 1)]; //drop newline char

	println!("hair color to verify : ");
	let mut hair_color = String::new();
	stdin().read_line(&mut hair_color).unwrap();
	let hair_color = &hair_color[..(hair_color.len() - 1)]; //drop newline char 
	verify_user(&connection, name, hair_color);	
}
