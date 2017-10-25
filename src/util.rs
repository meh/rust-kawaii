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
use traits::Kawaii;
use document::{Document, Indent, When};

/// An empty document.
pub fn empty() -> Rc<Document> {
	Rc::new(Document::Empty)
}

/// A string document.
pub fn string<T: Into<String>>(value: T) -> Rc<Document> {
	Rc::new(Document::String(value.into()))
}

/// Concatenate multiple documents.
pub fn concat<K: Kawaii, I: IntoIterator<Item = K>>(values: I) -> Rc<Document> {
	let mut values = values.into_iter();

	let init = if let Some(value) = values.next() {
		value.document(&Default::default())
	}
	else {
		return empty();
	};

	values.fold(init, |acc, item| Rc::new(Document::Cons {
		left: acc,
		right: item.document(&Default::default()),
	}))
}

/// Style a document.
pub fn style<K: Kawaii>(value: K, style: Style) -> Rc<Document> {
	Rc::new(Document::Style {
		inner: value.document(&Default::default()),
		style: style,
	})
}

/// Nests the given document.
pub fn nest<K: Kawaii>(value: K, indent: Indent, when: When) -> Rc<Document> {
	Rc::new(Document::Nest {
		inner: value.document(&Default::default()),
		indent: indent,
		when: when,
	})
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
		assert_eq!(kawaii::string("foo"),
			Rc::new(Document::String("foo".into())));
	}

	#[test]
	fn concat() {
		assert_eq!(kawaii::concat(&[kawaii::string("foo"), kawaii::string("bar")]),
			Rc::new(Document::Cons {
				left: Rc::new(Document::String("foo".into())),
				right: Rc::new(Document::String("bar".into())),
			}));
	}
}
