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
use std::collections::{LinkedList, VecDeque, BinaryHeap};
use std::collections::{HashMap, BTreeMap};
use std::collections::{HashSet, BTreeSet};
use std::hash::Hash;

use document::Document;
use config::{Config, Base, Precision, Syntax, Separator};

pub trait Kawaii {
	fn document(&self, config: &Config) -> Rc<Document>;
}

impl Kawaii for Rc<Document> {
	fn document(&self, _: &Config) -> Rc<Document> {
		Rc::clone(self)
	}
}

impl<'a, T: Kawaii + ?Sized> Kawaii for &'a T {
	fn document(&self, c: &Config) -> Rc<Document> {
		(*self).document(c)
	}
}

#[macro_export]
macro_rules! integer {
	($name:ty) => (
		impl Kawaii for $name {
			fn document(&self, c: &Config) -> Rc<Document> {
				let string = match c.get::<Base>().map(|b| *b).unwrap_or(Base::default()) {
					Base::Binary =>
						c.raw(format!("{:b}", self)),

					Base::Octal =>
						c.raw(format!("{:o}", self)),

					Base::Decimal =>
						c.raw(format!("{}", self)),

					Base::Hexadecimal =>
						c.raw(format!("{:0X}", self)),
				};

				if let Some(style) = c.get::<Syntax>()
					.and_then(|s| s.get("integer").or(s.get("number")))
				{
					c.style(string, *style)
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

#[macro_export]
macro_rules! float {
	($name:ty) => (
		impl Kawaii for $name {
			fn document(&self, c: &Config) -> Rc<Document> {
				let string = if let Some(&Precision(size)) = c.get::<Precision>() {
					c.raw(format!("{:.1$}", self, size))
				}
				else {
					c.raw(format!("{}", self))
				};

				if let Some(style) = c.get::<Syntax>()
					.and_then(|s| s.get("float").or(s.get("number")))
				{
					c.style(string, *style)
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

#[macro_export]
macro_rules! string {
	($name:ty) => (
		impl Kawaii for $name {
			fn document(&self, c: &Config) -> Rc<Document> {
				let string: &str = self.as_ref();
				let item = c.raw(format!("{:?}", string));

				if let Some(style) = c.get::<Syntax>()
					.and_then(|s| s.get("string"))
				{
					c.style(item, *style)
				}
				else {
					item
				}
			}
		}
	);

	($name:ty, $($rest:ty),*) => (
		string!($name);
		string!($($rest),*);
	);
}

#[macro_export]
macro_rules! list {
	($name:ty; $($params:tt)+) => (
		impl<$($params)*> $crate::Kawaii for $name {
			fn document(&self, c: &$crate::Config) -> ::std::rc::Rc<$crate::Document> {
				let $crate::config::Separator(separator) = c
					.get::<$crate::config::Separator>()
					.cloned().unwrap_or_default();

				let (left, separator, right) = if let Some(style) = c
					.get::<$crate::config::Syntax>()
					.and_then(|s| s.get("list"))
				{
					(c.style(c.raw("["), *style),
					 c.style(separator, *style),
					 c.style(c.raw("]"), *style))
				}
				else {
					(c.raw("["), separator, c.raw("]"))
				};

				c.sequence(&[left, c.iterator(self.iter(), separator), right])
			}
		}
	);
}

#[macro_export]
macro_rules! map {
	($name:ty; $($params:tt)+) => (
		impl<$($params)*> $crate::Kawaii for $name {
			fn document(&self, c: &$crate::Config) -> Rc<$crate::Document> {
				let $crate::config::Separator(separator) = c
					.get::<$crate::config::Separator>()
					.cloned().unwrap_or_default();

				let (left, separator, right) = if let Some(style) = c
					.get::<$crate::config::Syntax>()
					.and_then(|s| s.get("map"))
				{
					(c.style(c.raw("%{"), *style),
					 c.style(separator, *style),
					 c.style(c.raw("}"), *style))
				}
				else {
					(c.raw("%{"), separator, c.raw("}"))
				};

				let iter = self.iter().map(|(k, v)| {
					let separator = if let Some(style) = c
						.get::<$crate::config::Syntax>()
						.and_then(|s| s.get("map"))
					{
						c.style(c.raw(" => "), *style)
					}
					else {
						c.raw(" => ")
					};

					c.sequence(&[k.document(c), separator, v.document(c)])
				});

				c.sequence(&[left, c.iterator(iter, separator), right])
			}
		}
	);
}

#[macro_export]
macro_rules! set {
	($name:ty; $($params:tt)+) => (
		impl<$($params)*> Kawaii for $name {
			fn document(&self, c: &$crate::Config) -> ::std::rc::Rc<$crate::Document> {
				let $crate::config::Separator(separator) = c
					.get::<$crate::config::Separator>()
					.cloned().unwrap_or_default();

				let (left, separator, right) = if let Some(style) = c
					.get::<$crate::config::Syntax>()
					.and_then(|s| s.get("list"))
				{
					(c.style(c.raw("#{"), *style),
					 c.style(separator, *style),
					 c.style(c.raw("}"), *style))
				}
				else {
					(c.raw("#{"), separator, c.raw("}"))
				};

				c.sequence(&[left, c.iterator(self.iter(), separator), right])
			}
		}
	);
}

macro_rules! array {
	($size:tt) => (
		list!([T; $size]; T: Kawaii);
	);

	($size:tt, $($rest:tt),*) => (
		array!($size);
		array!($($rest),*);
	)
}

macro_rules! tuple {
	() => ();

	(($idx:tt => $typ:ident), $(($nidx:tt => $ntyp:ident),)* ) => {
		impl<$typ, $( $ntyp ),*> Kawaii for ($typ, $($ntyp),*)
			where $typ: Kawaii, $($ntyp: Kawaii),*
		{
			fn document(&self, c: &Config) -> Rc<Document> {
				let parts: &[&Kawaii] = &[&self.$idx, $(&self.$nidx),*];

				let Separator(separator) = c.get::<Separator>()
					.cloned().unwrap_or_default();

				let (left, separator, right) = if let Some(style) = c.get::<Syntax>()
					.and_then(|s| s.get("tuple"))
				{
					(c.style(c.raw("("), *style),
					 c.style(separator, *style),
					 c.style(c.raw(")"), *style))
				}
				else {
					(c.raw("("), separator, c.raw(")"))
				};

				c.sequence(&[left, c.iterator(parts.iter(), separator), right])
			}
		}

	  tuple!($(($nidx => $ntyp),)*);
	};
}

integer!(u8, i8, u16, i16, u32, i32, u64, i64);

float!(f32, f64);

string!(String, str);

array!( 0,  1,  2,  3,  4,  5,  6,  7,  8,  9);
array!(10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
array!(20, 21, 22, 23, 24, 25, 26, 27, 28, 29);
array!(30, 31, 32, 33, 34, 35, 36, 37, 38, 39);
array!(40, 41, 42, 43, 44, 45, 46, 47, 48, 49);
array!(50, 51, 52, 53, 54, 55, 56, 57, 58, 59);
array!(60, 61, 62, 63, 64);

tuple!(
	(25 => Z),
	(24 => Y),
	(23 => X),
	(22 => W),
	(21 => V),
	(20 => U),
	(19 => T),
	(18 => S),
	(17 => R),
	(16 => Q),
	(15 => P),
	(14 => O),
	(13 => N),
	(12 => M),
	(11 => L),
	(10 => K),
	(9  => J),
	(8  => I),
	(7  => H),
	(6  => G),
	(5  => F),
	(4  => E),
	(3  => D),
	(2  => C),
	(1  => B),
	(0  => A),
);

list!(Vec<T>; T: Kawaii);
list!(VecDeque<T>; T: Kawaii);
list!(LinkedList<T>; T: Kawaii);
list!(BinaryHeap<T>; T: Ord + Kawaii);
list!([T]; T: Kawaii);

map!(HashMap<K, V>; K: Eq + Hash + Kawaii, V: Kawaii);
map!(BTreeMap<K, V>; K: Kawaii, V: Kawaii);

set!(HashSet<T>; T: Eq + Hash + Kawaii);
set!(BTreeSet<T>; T: Kawaii);
