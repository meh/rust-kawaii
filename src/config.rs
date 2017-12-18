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
use std::slice;

use typemap::{Key, DebugMap, DebugAny};
use fnv::FnvHashMap;
use ansi_term::Style;
use document::Document;

#[derive(Debug)]
pub struct Config(DebugMap);

impl Default for Config {
	fn default() -> Self {
		Config(DebugMap::custom())
	}
}

impl Config {
	pub fn set<T: Key<Value = T> + DebugAny>(mut self, value: T) -> Self {
		self.0.insert::<T>(value);
		self
	}

	pub fn get<T: Key<Value = T> + DebugAny>(&self) -> Option<&T> {
		self.0.get::<T>()
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Base {
	Binary,
	Octal,
	Decimal,
	Hexadecimal,
}

impl Default for Base {
	fn default() -> Self {
		Base::Decimal
	}
}

impl Key for Base {
	type Value = Self;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Precision(pub usize);

impl Key for Precision {
	type Value = Self;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Limit(pub usize);

impl Key for Limit {
	type Value = Self;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Separator(pub Rc<Document>);

impl Default for Separator {
	fn default() -> Self {
		Separator(Rc::new(Document::Text(",".into())))
	}
}

impl Key for Separator {
	type Value = Self;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Width(pub usize);

impl Key for Width {
	type Value = Self;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Unicode(pub bool);

impl Default for Unicode {
	fn default() -> Self {
		Unicode(true)
	}
}

impl Key for Unicode {
	type Value = Self;
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Syntax(FnvHashMap<String, Rc<Style>>);

impl Syntax {
	pub fn set<N: AsRef<str>>(mut self, name: N, style: Style) -> Self {
		self.0.insert(name.as_ref().into(), style.into());
		self
	}

	pub fn get<N: AsRef<str>>(&self, name: N) -> Option<&Rc<Style>> {
		self.0.get(name.as_ref())
	}
}

impl Key for Syntax {
	type Value = Self;
}

#[derive(Default, Debug)]
pub struct For(FnvHashMap<String, Rc<Config>>);

impl For {
	pub fn set<N: AsRef<str>, C: Into<Rc<Config>>>(mut self, name: N, value: C) -> Self {
		self.0.insert(name.as_ref().into(), value.into());
		self
	}

	pub fn get<N: AsRef<str>>(&self, name: N) -> Option<&Rc<Config>> {
		self.0.get(name.as_ref())
	}
}

impl Key for For {
	type Value = Self;
}

#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Ignore(Vec<String>);

impl Ignore {
	pub fn add<N: AsRef<str>>(mut self, name: N) -> Self {
		self.0.push(name.as_ref().into());
		self
	}

	pub fn iter(&self) -> slice::Iter<String> {
		self.0.iter()
	}
}

impl Key for Ignore {
	type Value = Self;
}
