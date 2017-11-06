extern crate kawaii;

fn main() {
	use std::collections::{HashMap, HashSet, VecDeque};

	#[cfg(feature = "unstable")]
	struct Foo;

	#[derive(Debug)]
	struct Bar(i32);

	println!("{}", kawaii::inspect(23));
	println!("{}", kawaii::inspect("foobar"));
	println!("{}", kawaii::inspect(vec![1u8, 2, 3]));
	println!("{}", kawaii::inspect({
		let mut queue = VecDeque::new();
		queue.push_front("foo");
		queue.push_front("bar");
		queue
	}));
	println!("{}", kawaii::inspect([1u8, 2, 3]));
	println!("{}", kawaii::inspect((2, 3.5, "foobar")));
	println!("{}", kawaii::inspect({
		let mut map = HashMap::new();
		map.insert("foo", 42);
		map.insert("bar", 23);
		map
	}));
	println!("{}", kawaii::inspect({
		let mut set = HashSet::new();
		set.insert("foo");
		set.insert("bar");
		set
	}));

	println!("{}", kawaii::debug(&Bar(42)));

	#[cfg(feature = "unstable")]
	println!("{}", kawaii::inspect(Foo));
}
