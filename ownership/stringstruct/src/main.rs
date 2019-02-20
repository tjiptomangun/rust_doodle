//https://stackoverflow.com/questions/25754863/rust-how-to-create-struct-with-string-members

struct Foo {
	bar: String,
	baz: &'static str,
}

struct FooBar<'a> {
	foo: String,
	bar: &'a str,
}

fn main() {
	let mut foo = Foo {
		bar: "bar".to_string(),
		baz: "baz",
	};

	println!("foo {}, {}", foo.bar, foo.baz);

	foo.baz = "a";

	println!("foo {}, {}", foo.bar, foo.baz);

	let mut foo_bar = FooBar {
		foo : "foo".to_string(),
		bar: "bar",
	};

	println!("foo_bar {}, {}", foo_bar.foo, foo_bar.bar);

	foo_bar.bar = "a";

	println!("foo_bar {}, {}", foo_bar.foo, foo_bar.bar);
}
