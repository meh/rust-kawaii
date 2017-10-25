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

#![cfg_attr(feature = "unstable",
	feature(specialization, core_intrinsics))]

extern crate typemap;

pub extern crate ansi_term;
pub use ansi_term::{Color, Style};

mod traits;
pub use traits::Kawaii;

pub mod fmt;

pub mod config;
pub use config::Config;

pub mod document;
pub use document::Document;

pub mod util;
pub use util::*;

use std::rc::Rc;

pub fn print<T: Kawaii>(value: T) {
	println!("{}", inspect(value))
}

pub fn inspect<T: Kawaii>(value: T) -> Rc<Document> {
	value.document(&Config::default()
		.set(config::Syntax::default()
			.set("string", Color::Green.normal())
			.set("boolean", Color::Purple.normal())
			.set("number", Color::Yellow.normal())))
}
