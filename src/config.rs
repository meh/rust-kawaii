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

use std::collections::HashMap;

use typemap::{Key, DebugMap, DebugAny};
use ansi_term::Style;

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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Pretty(pub bool);

impl Key for Pretty {
	type Value = Self;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Width(pub usize);

impl Key for Width {
	type Value = Self;
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Syntax(HashMap<String, Style>);

impl Syntax {
	pub fn set<N: AsRef<str>>(mut self, name: N, style: Style) -> Self {
		self.0.insert(name.as_ref().into(), style);
		self
	}

	pub fn get<N: AsRef<str>>(&self, name: N) -> Option<&Style> {
		self.0.get(name.as_ref())
	}
}

impl Key for Syntax {
	type Value = Self;
}
