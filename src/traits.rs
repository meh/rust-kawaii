//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::rc::Rc;

use document::Document;
use config::Config;

pub trait Kawaii {
	fn document(&self, config: &Config) -> Rc<Document>;
}

#[cfg(feature = "unstable")]
mod magic {
	use std::rc::Rc;
	use std::fmt;
	use std::intrinsics;

	use document::Document;
	use config::Config;
	use traits::Kawaii;

	default impl<T: 'static> Kawaii for T {
		fn document(&self, _: &Config) -> Rc<Document> {
			use util::*;
			string(unsafe { intrinsics::type_name::<T>() })
		}
	}

	default impl<T: fmt::Debug> Kawaii for T {
		fn document(&self, _: &Config) -> Rc<Document> {
			use util::*;
			string(format!("{:?}", self))
		}
	}
}

impl Kawaii for Rc<Document> {
	fn document(&self, _: &Config) -> Rc<Document> {
		Rc::clone(self)
	}
}

#[macro_export]
macro_rules! integer {
	($name:ty) => (
		impl Kawaii for $name {
			fn document(&self, config: &Config) -> Rc<Document> {
				use config::{Base, Syntax};
				use util::*;

				let string = match config.get::<Base>().map(|b| *b).unwrap_or(Base::default()) {
					Base::Binary =>
						string(format!("{:b}", self)),

					Base::Octal =>
						string(format!("{:o}", self)),

					Base::Decimal =>
						string(format!("{}", self)),

					Base::Hexadecimal =>
						string(format!("{:0X}", self)),
				};

				if let Some(syntax) = config.get::<Syntax>()
					.and_then(|s| s.get("integer").or(s.get("number")))
				{
					style(string, *syntax)
				}
				else {
					string
				}
			}
		}
	);

	($name:ty, $($rest:ty),*) => (
		integer!($name);
		integer!($($rest),*);
	);
}

integer!(u8, i8, u16, i16, u32, i32, u64, i64);

#[macro_export]
macro_rules! float {
	($name:ty) => (
		impl Kawaii for $name {
			fn document(&self, config: &Config) -> Rc<Document> {
				use config::{Precision, Syntax};
				use util::*;

				let string = if let Some(&Precision(size)) = config.get::<Precision>() {
					string(format!("{:.1$}", self, size))
				}
				else {
					string(format!("{}", self))
				};

				if let Some(syntax) = config.get::<Syntax>()
					.and_then(|s| s.get("float").or(s.get("number")))
				{
					style(string, *syntax)
				}
				else {
					string
				}
			}
		}
	);

	($name:ty, $($rest:ty),*) => (
		float!($name);
		float!($($rest),*);
	);
}

float!(f32, f64);

#[macro_export]
macro_rules! string {
	($name:ty) => (
		impl Kawaii for $name {
			fn document(&self, config: &Config) -> Rc<Document> {
				use config::Syntax;
				use util::*;

				let string = string(format!("{:?}", self));

				if let Some(syntax) = config.get::<Syntax>()
					.and_then(|s| s.get("string"))
				{
					style(string, *syntax)
				}
				else {
					string
				}
			}
		}

		impl<'a> Kawaii for &'a $name {
			fn document(&self, config: &Config) -> Rc<Document> {
				(*self).document(config)
			}
		}
	);

	($name:ty, $($rest:ty),*) => (
		string!($name);
		string!($($rest),*);
	);
}

string!(String, str);

//#[macro_export]
//macro_rules! list {
//	($($params:tt)* ; $name:ty) => (
//		impl<$($params)*> Kawaii for $name {
//			fn document(&self, config: &Config) -> Rc<Document> {
//				unimplemented!();
//			}
//		}
//	);
//}
//
//list!(T: Kawaii; Vec<T>);
//list!('a, T: Kawaii; &'a [T]);
