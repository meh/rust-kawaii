use itertools::Itertools;
use ansi_term::Style;

use syn;
use syn::MetaItem::{List, NameValue, Word};
use syn::NestedMetaItem::{Literal, MetaItem};

use error;
use parse;

#[derive(Default, Debug)]
pub struct Container {
	style: Option<Style>,
}

#[derive(Default, Debug)]
pub struct Field {
	style: Option<Style>,
	ignore: bool,
	debug: bool,
	unknown: bool,
}

#[derive(Default, Debug)]
pub struct Variant {
	style: Option<Style>,
}

impl Container {
	pub fn parse(item: &syn::DeriveInput) -> error::Result<Container> {
		let mut container = Container::default();

		for item in item.attrs.iter().filter_map(parse::attributes).flatten() {
			match *item {
				// #[kawaii(style = "...")]
				MetaItem(NameValue(ref name, ref lit)) if name == "style" => {
					container.style = Some(parse::style(parse::string(lit)?)?);
				}

				MetaItem(ref item) => {
					return Err(format!("unknown kawaii container attribute `{}`", item.name()).into());
				}

				Literal(_) => {
					return Err("unexpected literal in kawaii container attribute".into());
				}
			}
		}

		Ok(container)
	}
}

impl Field {
	pub fn parse(item: &syn::Field) -> error::Result<Field> {
		let mut field = Field::default();

		for item in item.attrs.iter().filter_map(parse::attributes).flatten() {
			match *item {
				// #[kawaii(style = "...")]
				MetaItem(NameValue(ref name, ref lit)) if name == "style" => {
					field.style = Some(parse::style(parse::string(lit)?)?);
				}

				// #[kawaii(ignore)]
				MetaItem(Word(ref name)) if name == "ignore" => {
					field.ignore = true;
				}

				// #[kawaii(debug)]
				MetaItem(Word(ref name)) if name == "debug" => {
					field.debug = true;
				}

				// #[kawaii(unknown)]
				MetaItem(Word(ref name)) if name == "unknown" => {
					field.unknown = true;
				}

				MetaItem(ref item) => {
					return Err(format!("unknown kawaii field attribute `{}`", item.name()).into());
				}

				Literal(_) => {
					return Err("unexpected literal in kawaii field attribute".into());
				}
			}
		}

		Ok(field)
	}
}

impl Variant {
	pub fn parse(item: &syn::Variant) -> error::Result<Variant> {
		let mut variant = Variant::default();

		for item in item.attrs.iter().filter_map(parse::attributes).flatten() {
			match *item {
				// #[kawaii(style = "...")]
				MetaItem(NameValue(ref name, ref lit)) if name == "style" => {
					variant.style = Some(parse::style(parse::string(lit)?)?);
				}

				MetaItem(ref item) => {
					return Err(format!("unknown kawaii variant attribute `{}`", item.name()).into());
				}

				Literal(_) => {
					return Err("unexpected literal in kawaii variant attribute".into());
				}
			}
		}

		Ok(variant)
	}
}
