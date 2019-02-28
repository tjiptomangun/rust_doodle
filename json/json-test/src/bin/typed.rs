extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
struct Person {
	name: String,
	age: u8,
	phones: Vec<String>,
}


fn typed_example() -> Result<()> {
	//Some JSON input data as &str. Maybe this comes from the user

	let data = r#"
	{
		"name": "John Doe",
		"age": 43,
		"phones": [
			"+44 1234567",
			"+44 2345678"
		]
	}"#;

	// Parse the string of data into a Person object.
	// This is exactly the same function as one of the
	// produced serde_json::Value aboce, but
	// now we are asking it for a person as output.
	let p: Person = serde_json::from_str(data)?;

	// Access parts of data by indexing with square brackets
	println!("Please call {} at the number {}", p.name , p.phones[0]);
	Ok(())
}

fn main() -> Result <()> { 
	typed_example()
}
