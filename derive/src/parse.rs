use std::collections::VecDeque;

use syn;
use syn::MetaItem::{List, NameValue, Word};
use syn::NestedMetaItem::{Literal, MetaItem};

use ansi_term::{Color, Style};

use error;

pub fn string(lit: &syn::Lit) -> error::Result<&str> {
	match lit {
		&syn::Lit::Str(ref s, _) =>
			Ok(s),

		_ =>
			Err("expected string literal".into())
	}
}

pub fn path(lit: &syn::Lit) -> error::Result<syn::Path> {
	Ok(syn::parse_path(string(lit)?)?)
}

pub fn attributes(attr: &syn::Attribute) -> Option<&[syn::NestedMetaItem]> {
	match attr.value {
		List(ref name, ref items) if name == "kawaii" =>
			Some(&items),

		_ =>
			None,
	}
}

pub fn style(s: &str) -> error::Result<Style> {
	fn color(s: &str) -> error::Result<Color> {
		Ok(match s {
			"black"  => Color::Black,
			"red"    => Color::Red,
			"green"  => Color::Green,
			"yellow" => Color::Yellow,
			"blue"   => Color::Blue,
			"purple" => Color::Purple,
			"cyan"   => Color::Cyan,
			"white"  => Color::White,

			color if color.starts_with("#") && color.len() == 7 =>
				Color::RGB(
					u8::from_str_radix(&color[1..3], 16)?,
					u8::from_str_radix(&color[4..5], 16)?,
					u8::from_str_radix(&color[5..7], 16)?),

			color if u8::from_str_radix(color, 10).is_ok() =>
				Color::Fixed(u8::from_str_radix(color, 10)?),

			_ =>
				return Err("not a color".into())
		})
	}

	let mut style = Style::new();
	let mut parts = s.split(':').map(str::to_lowercase).collect::<VecDeque<_>>();

	// Foreground color if any.
	match parts.pop_front() {
		Some(ref s) if s.is_empty() =>
			(),

		Some(ref s) =>
			style = style.fg(color(s)?),

		None =>
			return Ok(style)
	}

	// Background color, optional.
	match parts.pop_front() {
		Some(ref s) if color(s).is_ok() =>
			style = style.on(color(s).unwrap()),

		Some(s) =>
			parts.push_front(s),

		None =>
			return Ok(style)
	}

	// Attributes.
	for attr in parts {
		match attr.as_ref() {
			"bold" =>
				style = style.bold(),

			"dimmed" =>
				style = style.dimmed(),

			"italic" =>
				style = style.italic(),

			"underline" =>
				style = style.underline(),

			"blink" =>
				style = style.blink(),

			"reverse" =>
				style = style.reverse(),

			"hidden" =>
				style = style.hidden(),

			"strikethrough" =>
				style = style.strikethrough(),

			name =>
				return Err(format!("unknown style {}", name).into())
		}
	}

	Ok(style)
}
