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
use std::fmt;

use ansi_term::Style;
use config::Config;

#[derive(Debug)]
pub enum Document {
	Empty,
	Line,

	String(String),

	With {
		inner: Rc<Document>,
		config: Config,
	},

	Cons {
		left: Rc<Document>,
		right: Rc<Document>
	},

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

	Fits {
		inner: Rc<Document>,
		mode: Fits,
	},

	Force(Rc<Document>),
	Collapse(usize),

	Style {
		inner: Rc<Document>,
		style: Style,
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
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Group {
	Inherit,
	This,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Fits {
	Enabled,
	Disabled,
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

impl fmt::Display for Document {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		::fmt::document(f, self)
	}
}
