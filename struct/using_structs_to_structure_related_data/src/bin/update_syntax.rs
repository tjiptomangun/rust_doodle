struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

//struct without name
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
	let user1 = User{
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	let user2 = User{
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		..user1
	};
}
