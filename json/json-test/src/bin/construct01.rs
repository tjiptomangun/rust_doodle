use serde_json::json; 

fn main() {
	let full_name = "John Doe"; 
	let age_last_year = 42;

	let john = json!({
		"name": full_name,
		"age": age_last_year + 1,
		"phones": [
			"+44 1234567",
			"+44 2345678",

		]
	});

	println!("first phone number: {}", john["phones"][0]);

	println!("{}", john.to_string());
}
