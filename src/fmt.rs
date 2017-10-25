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

use std::fmt;

use document::{Document};
use config::{Width, Config};

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
}

pub fn document(f: &mut fmt::Formatter, mut doc: &Document) -> fmt::Result {
	let mut state = State {
		width: None,
		mode: Mode::Break,
		indent: 0,
	};

	if let &Document::With { ref inner, ref config } = doc {
		if let Some(&Width(width)) = config.get::<Width>() {
			state.width = Some(width);
		}

		doc = inner;
	}

	it(f, doc, state)
}

fn it(f: &mut fmt::Formatter, doc: &Document, state: State) -> fmt::Result {
	fn indent(f: &mut fmt::Formatter, n: usize) -> fmt::Result {
		Err(fmt::Error)
	}

	match *doc {
		Document::Empty =>
			(),

		Document::Line =>
			indent(f, state.indent)?,

		Document::String(ref string) =>
			f.write_str(string)?,

		Document::Style { ref inner, style } => {
			write!(f, "{}", style.prefix())?;
			it(f, inner, state)?;
			write!(f, "{}", style.suffix())?;
		}

		ref doc =>
			unimplemented!("{:?}", doc)
	}

	Ok(())
}
