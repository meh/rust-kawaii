extern crate ansi_term;

extern crate proc_macro;
use proc_macro::TokenStream;

#[macro_use]
extern crate quote;
extern crate syn;

#[macro_use]
extern crate nom;
extern crate itertools;

mod parse;
mod error;
mod attr;
mod ast;
mod derive;

#[proc_macro_derive(Kawaii, attributes(kawaii))]
pub fn kawaii(input: TokenStream) -> TokenStream {
	let input = input.to_string();
	let input = syn::parse_derive_input(&input).unwrap();

	match derive_kawaii(&input) {
		Ok(expanded) =>
			expanded.parse().unwrap(),

		Err(error) =>
			panic!(error.0),
	}
}

fn derive_kawaii(input: &syn::DeriveInput) -> error::Result<quote::Tokens> {
	let ident = &input.ident;
	let dummy = syn::Ident::new(format!("_IMPL_KAWAII_FOR_{}", ident));
	let container = ast::Container::parse(input)?;
	let (impl_generics, ty_generics, where_clause) =
		container.generics.split_for_impl();
	let unstable = if cfg!(unstable) {
		quote! { default }
	}
	else {
		quote! { }
	};

	println!("{:#?}", container);

	Ok(quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy: () = {
			extern crate kawaii;
			use std::rc::Rc;
			use kawaii::{Kawaii, Config, Document, util};

			#[automatically_derived]
			#unstable impl #impl_generics Kawaii for #ident #ty_generics #where_clause {
				fn document(&self, c: &Config) -> Rc<Document> {
					#container
				}
			}
		};
	})
}
