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
use itertools::Itertools;
use traits::Kawaii;
use config::{self, Config, Syntax, Separator};
use document::{Document, Indent, When, Break, Group};

impl Config {
	/// An empty document.
	pub fn empty(&self) -> Rc<Document> {
		Rc::new(Document::Empty)
	}

	/// Line document.
	pub fn line(&self) -> Rc<Document> {
		Rc::new(Document::Line)
	}

	/// A string document.
	pub fn text<T: Into<String>>(&self, value: T) -> Rc<Document> {
		Rc::new(Document::Text(value.into()))
	}

	/// Breaking point.
	pub fn break_with<T: Into<String>>(&self, value: T, mode: Break) -> Rc<Document> {
		Rc::new(Document::Break {
			value: value.into(),
			mode: mode,
		})
	}

	/// Concatenate multiple documents.
	pub fn sequence<K: Kawaii, I: IntoIterator<Item = K>>(&self, values: I) -> Rc<Document> {
		Rc::new(Document::Sequence(values.into_iter()
			.map(|k| k.document(self)).collect()))
	}

	/// Create a new named group.
	pub fn group<K: Kawaii, N: AsRef<str>>(&self, name: N, mode: Group, value: K) -> Rc<Document> {
		Rc::new(Document::Group {
			name: name.as_ref().into(),
			inner: value.document(self),
			mode: mode,
		})
	}

	/// Nests the given document.
	pub fn nest<K: Kawaii>(&self, value: K, indent: Indent, when: When) -> Rc<Document> {
		Rc::new(Document::Nest {
			inner: value.document(self),
			indent: indent,
			when: when,
		})
	}

	/// Style a document.
	pub fn style<K: Kawaii, S: Into<Rc<Style>>>(&self, value: K, style: S) -> Rc<Document> {
		Rc::new(Document::Style {
			inner: value.document(self),
			style: style.into(),
		})
	}

	/// Creates a document out of an iterator.
	pub fn iterator<S: Kawaii, K: Kawaii, T: IntoIterator<Item = K>>(&self, values: T, separator: S) -> Rc<Document> {
		let mode = self.get::<Break>().cloned().unwrap_or_default();
		let separator = separator.document(self);

		self.sequence(values.into_iter()
			.map(|v| v.document(self))
			.intersperse(separator))
	}

	/// Create a document from its debug representation.
	///
	/// TODO(meh): parse the output and pretty print it
	pub fn debug<T: fmt::Debug>(&self, value: &T) -> Rc<Document> {
		let item = self.text(format!("{:?}", value));

		if let Some(style) = self.get::<Syntax>().and_then(|s| s.get("debug")) {
			self.style(item, Rc::clone(style))
		}
		else {
			item
		}
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use util as kawaii;
	use traits::Kawaii;
	use document::Document;

	#[test]
	fn empty() {
		let c = Config::default();

		assert_eq!(c.empty(), Rc::new(Document::Empty));
	}

	#[test]
	fn text() {
		let c = Config::default();

		assert_eq!(c.text("foo"),
			Rc::new(Document::Text("foo".into())));
	}

	#[test]
	fn sequence() {
		let c = Config::default();

		assert_eq!(kawaii::sequence(&[c.text("foo"), c.text("bar")]),
			Rc::new(Document::Sequence(vec![
				Rc::new(Document::Text("foo".into())),
				Rc::new(Document::Text("bar".into())),
			])));
	}
}
