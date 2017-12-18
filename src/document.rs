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

use ansi_term::Style;
use typemap::Key;

/// Algebraic document.
#[derive(PartialEq, Clone, Debug)]
pub enum Document {
	Empty,
	Line,

	Text(String),

	Sequence(Vec<Rc<Document>>),

	Nest {
		inner: Rc<Document>,
		indent: Indent,
		when: When,
	},

	Break {
		value: String,
		mode: Break,
	},

	Group {
		name: String,
		inner: Rc<Document>,
		mode: Group,
	},

	Style {
		inner: Rc<Document>,
		style: Rc<Style>,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Indent {
	Cursor,
	Reset,
	Value(usize),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum When {
	Always,
	Break,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Break {
	Flex,
	Strict,
	Maybe,
}

impl Default for Break {
	fn default() -> Self {
		Break::Maybe
	}
}

impl Key for Break {
	type Value = Self;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Group {
	Inherit,
	This,
}

impl Default for Document {
	fn default() -> Document {
		Document::Empty
	}
}

impl Default for When {
	fn default() -> When {
		When::Always
	}
}
