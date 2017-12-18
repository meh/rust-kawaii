use syn;
use attr;
use error;

#[derive(Debug)]
pub struct Container<'a> {
	pub ident: syn::Ident,
	pub attr: attr::Container,
	pub body: Body<'a>,
	pub generics: &'a syn::Generics,
}

#[derive(Debug)]
pub enum Body<'a> {
	Struct(Style, Vec<Field<'a>>),
	Enum(Vec<Variant<'a>>),
}

#[derive(Debug)]
pub struct Variant<'a> {
	pub ident: syn::Ident,
	pub attr: attr::Variant,
	pub style: Style,
	pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Field<'a> {
	pub ident: Option<syn::Ident>,
	pub attr: attr::Field,
	pub ty: &'a syn::Ty,
}

#[derive(Copy, Clone, Debug)]
pub enum Style {
	Struct,
	Tuple,
	NewType,
	Unit,
}

impl<'a> Container<'a> {
	pub fn parse(item: &'a syn::DeriveInput) -> error::Result<Container<'a>> {
		Ok(Container {
			ident: item.ident.clone(),
			generics: &item.generics,
			attr: attr::Container::parse(item)?,
			body: match item.body {
				syn::Body::Enum(ref variants) =>
					Body::Enum(variants.iter().map(Variant::parse).collect::<error::Result<_>>()?),

				syn::Body::Struct(ref variant) => {
					Body::Struct(Style::parse(variant), match *variant {
						syn::VariantData::Struct(ref fields) |
						syn::VariantData::Tuple(ref fields) =>
							fields.iter().map(Field::parse).collect::<error::Result<_>>()?,

						syn::VariantData::Unit =>
							Vec::new()
					})
				}
			}
		})
	}
}

impl<'a> Variant<'a> {
	pub fn parse(item: &'a syn::Variant) -> error::Result<Variant<'a>> {
		Ok(Variant {
			ident: item.ident.clone(),
			attr: attr::Variant::parse(item)?,
			style: Style::parse(&item.data),
			fields: match item.data {
				syn::VariantData::Struct(ref fields) |
				syn::VariantData::Tuple(ref fields) =>
					fields.iter().map(Field::parse).collect::<error::Result<_>>()?,

				syn::VariantData::Unit =>
					Vec::new()
			}
		})
	}
}

impl<'a> Field<'a> {
	pub fn parse(item: &'a syn::Field) -> error::Result<Field<'a>> {
		Ok(Field {
			ident: item.ident.clone(),
			ty: &item.ty,
			attr: attr::Field::parse(item)?,
		})
	}
}

impl Style {
	pub fn parse(item: &syn::VariantData) -> Style {
		match *item {
			syn::VariantData::Struct(_) =>
				Style::Struct,

			syn::VariantData::Tuple(ref fields) if fields.len() == 1 =>
				Style::NewType,

			syn::VariantData::Tuple(_) =>
				Style::Tuple,

			syn::VariantData::Unit =>
				Style::Unit
		}
	}
}
