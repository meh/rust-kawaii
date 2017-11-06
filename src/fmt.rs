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

use unicode_segmentation::UnicodeSegmentation;
use term_size as terminal;
use document::{Document, Indent, When, Break};
use config::{Config, Width};

/// Document with a configuration for printing.
#[derive(Debug)]
pub struct With {
	config: Config,
	document: Rc<Document>,
}

impl fmt::Display for Document {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		it(f, self, &State::default())
	}
}

impl fmt::Display for With {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut state = State::default();

		if let Some(&Width(width)) = self.config.get::<Width>() {
			state.width = Some(width);
		}
		else if let Some((width, _)) = terminal::dimensions() {
			state.width = Some(width);
		}

		it(f, &self.document, &state)
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
  /// A fitted document with breaks as breaks.
	Break,

  /// A fitted document with breaks as flats.
	Flat,

  /// A document being fitted that will cancel (fit) the next break.
	NextFits,

  /// A document being fitted that will not accept next break fits.
	NoFitting,
}

#[derive(Debug)]
struct State {
	width: Option<usize>,
	mode: Mode,
	indent: usize,
	cursor: usize,
}

impl Default for State {
	fn default() -> Self {
		State {
			width: None,
			mode: Mode::Flat,
			indent: 0,
			cursor: 0,
		}
	}
}

fn it(f: &mut fmt::Formatter, doc: &Document, state: &State) -> fmt::Result {
	#[inline(always)]
	fn indent(f: &mut fmt::Formatter, n: usize) -> fmt::Result {
		write!(f, "\n{:1$}", "", n)
	}

	#[inline(always)]
	fn fits(width: usize, cursor: usize, doc: &Document) -> bool {
		false
	}

	match *doc {
		Document::Empty =>
			(),

		Document::Line =>
			indent(f, state.indent)?,

		Document::Raw(ref string) =>
			f.write_str(string)?,

		Document::Sequence(ref values) => {
			for value in values {
				it(f, value, state)?;
			}
		}

		Document::Nest { ref inner, indent, when } if when == When::Always || (when == When::Break && state.mode == Mode::Break) =>
			it(f, inner, &State {
				indent: match indent {
					Indent::Cursor => state.indent,
					Indent::Reset => 0,
					Indent::Value(size) => state.indent + size,
				},

				.. *state })?,

		Document::Nest { ref inner, .. } =>
			it(f, inner, state)?,

		Document::Break { ref value, mode: Break::Flex } => {
			let cursor = state.cursor + value.len();
		}

		Document::Style { ref inner, style } => {
			write!(f, "{}", style.prefix())?;
			it(f, inner, state)?;
			write!(f, "{}", style.suffix())?;
		}

		_ => ()
	}

	Ok(())
}
