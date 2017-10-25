extern crate kawaii;
use kawaii::{Kawaii, Style, Config};

fn main() {
	struct Foo;

	kawaii::print(23);
	kawaii::print("foobar");
	kawaii::print(vec![1, 2, 3]);
	kawaii::print(Foo);
}
