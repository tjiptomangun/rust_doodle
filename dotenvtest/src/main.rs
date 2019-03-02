use std::env;

fn main() {
	match env::var("LANG") {
		Ok(lang) => println!("Language code: {}", lang),
		Err(er) => println!("Couldn't load read LANG ({})", er),
	};
}
