#[macro_use]
extern crate kawaii_derive;
extern crate kawaii;

#[derive(Kawaii)]
#[kawaii(style = "blue:bold")]
pub struct Struct {
	#[kawaii(like = "debug")]
	debug: Debug,

	#[kawaii(like = "unknown")]
	not_debug: NotDebug,

	#[kawaii(style = "15:red:bold:italic")]
	foo: u8,

	#[kawaii(ignore)]
	bar: String,
}

#[derive(Kawaii)]
#[kawaii(style = "yellow:italic")]
pub struct Tuple(
	#[kawaii(like = "debug")]
	Debug,

	#[kawaii(like = "unknown")]
	NotDebug,

	#[kawaii(style = "blue:green:underline")]
	u8,

	#[kawaii(ignore)]
	String,
);

#[derive(Kawaii)]
#[kawaii(style = "red:strikethrough")]
pub struct NewType(
	#[kawaii(style = "blue:green:underline")]
	u8,
);

#[derive(Kawaii)]
#[kawaii(style = "green:reverse")]
pub struct Unit;

#[derive(Kawaii)]
#[kawaii(style = ":bold")]
pub enum Enum {
	#[kawaii(style = "blue:bold")]
	Struct {
		#[kawaii(like = "debug")]
		debug: Debug,

		#[kawaii(like = "unknown")]
		not_debug: NotDebug,

		#[kawaii(style = "blue:green:underline")]
		foo: u8,

		#[kawaii(ignore)]
		bar: String,
	},

	#[kawaii(style = "yellow:italic")]
	Tuple(
		#[kawaii(like = "debug")]
		Debug,

		#[kawaii(like = "unknown")]
		NotDebug,

		#[kawaii(style = "blue:green:underline")]
		u8,

		#[kawaii(ignore)]
		String,
	),

	#[kawaii(style = "red:strikethrough")]
	NewType(
		#[kawaii(style = "blue:green:underline")]
		u8,
	),

	#[kawaii(style = "green:reverse")]
	Unit,
}

#[derive(Debug)]
pub struct Debug {
	foo: String,
	bar: Option<String>,
	baz: u32,
}

pub struct NotDebug {
	foo: i32,
}

fn main() {
	kawaii::print(Struct {
		debug: Debug {
			foo: "Foo".into(),
			bar: Some("Bar".into()),
			baz: 42,
		},

		not_debug: NotDebug {
			foo: 23,
		},

		foo: 7,
		bar: "baz".into(),
	});

	println!("{:?}", ::std::mem::size_of::<kawaii::Document>());

//	kawaii::print(Tuple(
//		Debug {
//			foo: "Foo".into(),
//			bar: Some("Bar".into()),
//			baz: 42,
//		},
//
//		NotDebug {
//			foo: 23,
//		},
//
//		7,
//		"baz".into(),
//	));
//
//	kawaii::print(NewType(42));
//	kawaii::print(Unit);
//
//	kawaii::print(Enum::Struct {
//		debug: Debug {
//			foo: "Foo".into(),
//			bar: Some("Bar".into()),
//			baz: 42,
//		},
//
//		not_debug: NotDebug {
//			foo: 23,
//		},
//
//		foo: 7,
//		bar: "baz".into(),
//	});
//
//	kawaii::print(Enum::Tuple(
//		Debug {
//			foo: "Foo".into(),
//			bar: Some("Bar".into()),
//			baz: 42,
//		},
//
//		NotDebug {
//			foo: 23,
//		},
//
//		7,
//		"baz".into(),
//	));
//
//	kawaii::print(Enum::NewType(42));
//	kawaii::print(Enum::Unit);
}
