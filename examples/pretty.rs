#[macro_use]
extern crate kawaii_derive;

extern crate kawaii;
use kawaii::Kawaii;

#[derive(Kawaii)]
#[kawaii(style = "blue:blink")]
pub struct Person {
	#[kawaii(debug)]
	name: Name,
	#[kawaii(style = "blue:green")]
	age: u8,
	#[kawaii(ignore)]
	job: String,
}

#[derive(Debug)]
pub struct Name {
	first: String,
	middle: Option<String>,
	last: String,
}

fn main() {
	let person = Person {
		name: Name {
			first: "Richard".into(),
			middle: Some("Matthew".into()),
			last: "Stallman".into(),
		},

		age: 64,
		job: "Preacher".into()
	};

	kawaii::print(&person);
}
