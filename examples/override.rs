#[macro_use]
extern crate kawaii_derive;
extern crate kawaii;

#[derive(Kawaii)]
pub struct Struct {
	foo: u32,
	bar: String,
	baz: Vec<u8>,
}

fn main() {
	println!("{}", kawaii::it(&Struct {
		foo: 42,
		bar: "fuffa".into(),
		baz: vec![1, 2, 3, 4],
	}).ignore("override::Struct::baz"));
}
