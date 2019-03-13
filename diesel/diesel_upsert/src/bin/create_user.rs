extern crate diesel_upsert;
extern crate diesel;

use self::diesel_upsert::*;
use std::io::stdin;

fn main() {
	let connection = establish_connection();

	println!("name please: ");
	let mut name= String::new();
	stdin().read_line(&mut name).unwrap();
	let name = &name[..(name.len() - 1)]; //drop newline char
	
	println!("hair color please : ");
	let mut hair_color= String::new();
	stdin().read_line(&mut hair_color).unwrap();
	let hair_color = &hair_color[..(hair_color.len() - 1)]; //drop newline char

	println!("minutes to expire please: ");
	let mut minutes_str= String::new();
	stdin().read_line(&mut minutes_str).unwrap();
	let minutes_str = &minutes_str[..(minutes_str.len() - 1)]; //drop newline char

	let min = minutes_str.parse::<i64>().unwrap();

	create_user(&connection, name, hair_color, min);

	println!("\nSaved user {} ", name);
	
}

// #[cfg(not(windows))]
// const EOF: &'static str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &'static str = "CTRL+Z"; 
