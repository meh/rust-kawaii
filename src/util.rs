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
use document::{Document, Indent, When, Break};

impl Config {
	/// An empty document.
	pub fn empty(&self) -> Rc<Document> {
		empty()
	}

	/// A string document.
	pub fn raw<T: Into<String>>(&self, value: T) -> Rc<Document> {
		raw(value)
	}

	/// Concatenate multiple documents.
	pub fn sequence<K: Kawaii, I: IntoIterator<Item = K>>(&self, values: I) -> Rc<Document> {
		Rc::new(Document::Sequence(values.into_iter()
			.map(|k| k.document(self)).collect()))
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
	pub fn style<K: Kawaii>(&self, value: K, style: Style) -> Rc<Document> {
		Rc::new(Document::Style {
			inner: value.document(self),
			style: style,
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
		let item = self.raw(format!("{:?}", value));

		if let Some(style) = self.get::<Syntax>().and_then(|s| s.get("debug")) {
			self.style(item, *style)
		}
		else {
			item
		}
	}
}

/// An empty document.
pub fn empty() -> Rc<Document> {
	Rc::new(Document::Empty)
}

/// A string document.
pub fn raw<T: Into<String>>(value: T) -> Rc<Document> {
	Rc::new(Document::Raw(value.into()))
}

/// Concatenate multiple documents.
pub fn sequence<K: Kawaii, I: IntoIterator<Item = K>>(values: I) -> Rc<Document> {
	Config::default().sequence(values)
}

/// Style a document.
pub fn style<K: Kawaii>(value: K, style: Style) -> Rc<Document> {
	Config::default().style(value, style)
}

/// Nests the given document.
pub fn nest<K: Kawaii>(value: K, indent: Indent, when: When) -> Rc<Document> {
	Config::default().nest(value, indent, when)
}

/// Creates a document out of an iterator.
pub fn iterator<S: Kawaii, K: Kawaii, T: IntoIterator<Item = K>>(values: T, separator: S) -> Rc<Document> {
	Config::default().iterator(values, separator)
}

/// Creates a debug document.
pub fn debug<T: fmt::Debug>(value: &T) -> Rc<Document> {
	Config::default().debug(value)
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use util as kawaii;
	use traits::Kawaii;
	use document::Document;

	#[test]
	fn empty() {
		assert_eq!(kawaii::empty(), Rc::new(Document::Empty));
	}

	#[test]
	fn string() {
		assert_eq!(kawaii::raw("foo"),
			Rc::new(Document::Raw("foo".into())));
	}

	#[test]
	fn sequence() {
		assert_eq!(kawaii::sequence(&[kawaii::raw("foo"), kawaii::raw("bar")]),
			Rc::new(Document::Sequence(vec![
				Rc::new(Document::Raw("foo".into())),
				Rc::new(Document::Raw("bar".into())),
			])));
	}
}
