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

#![allow(unused)]

extern crate itertools;
extern crate unicode_segmentation;

extern crate typemap;
extern crate fnv;

extern crate term_size;

pub extern crate ansi_term;
pub use ansi_term::{Color, Style};

mod traits;
pub use traits::Kawaii;

pub mod fmt;

pub mod config;
pub use config::Config;

pub mod document;
pub use document::Document;

#[macro_use]
pub mod util;
pub use util::*;

use std::rc::Rc;

pub fn default() -> Config {
	Config::default()
		.set(config::Syntax::default()
			.set("unknown", Color::Fixed(8).normal())
			.set("debug", Color::Fixed(15).normal())
			.set("boolean", Color::Purple.normal())
			.set("number", Color::Yellow.normal())
			.set("string", Color::Green.normal()))
}

pub fn print<T: Kawaii>(value: T) {
	eprintln!("{:#?}", inspect(&value));
	println!("{}", inspect(&value))
}

pub fn print_with<T: Kawaii>(value: T, config: &Config) {
	eprintln!("{:#?}", inspect_with(&value, config));
	println!("{}", inspect_with(&value, config))
}

pub fn inspect<T: Kawaii>(value: T) -> Rc<Document> {
	value.document(&default())
}

pub fn inspect_with<T: Kawaii>(value: T, config: &Config) -> Rc<Document> {
	value.document(config)
}
