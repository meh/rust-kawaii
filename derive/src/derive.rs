use ansi_term::{Style, Color};
use quote::{Tokens, ToTokens};
use syn;
use ast;
use attr;

impl<'a> ToTokens for ast::Container<'a> {
	fn to_tokens(&self, out: &mut Tokens) {
		let name = self.ident.to_string();
		let name = quote! { format!("{}::{}", module_path!(), #name) };

		out.append(match self.body {
			ast::Body::Struct(style, ref fields) => {
				from_struct(name, &self.attr, style, fields)
			}

			ast::Body::Enum(ref variants) => {
				from_enum(name, &self.attr, variants)
			}
		});
	}
}

fn block<F: FnOnce(&mut Tokens)>(f: F) -> Tokens {
	let mut out = Tokens::new();
	f(&mut out);

	quote! {
		{ #out }
	}
}

fn expr<F: FnOnce(&mut Tokens)>(f: F) -> Tokens {
	let mut out = Tokens::new();
	f(&mut out);
	out
}

fn repr<F: FnOnce(&mut Tokens)>(f: F) -> Tokens {
	block(|out| {
		let body = expr(f);

		out.append(quote! {
			let mut repr: Vec<Rc<Document>> = Vec::new();
			#body;
			c.sequence(repr)
		});
	})
}

fn from_struct(name: Tokens, attr: &attr::Container, style: ast::Style, fields: &[ast::Field]) -> Tokens {
	repr(|out| {
		// Print name.
		out.append(if let Some(style) = attr.style {
			let style = from_style(style);

			quote! {
				repr.push(c.style(c.text(#name), #style));
			}
		}
		else {
			quote! {
				repr.push(c.text(#name));
			}
		});

		// Derive fields.
		let fields = repr(|out| {
			let mut fields = fields.iter().filter(|f| !f.attr.ignore).peekable();

			while let Some(field) = fields.next() {
				let field = from_field(field);

				out.append(quote! {
					repr.push(#field);
				});

				// TODO(meh): insert breaks
				if fields.peek().is_some() {
					out.append(quote! {
						repr.push(c.text(","));
					});
				}
			}
		});

		// Print body.
		match style {
			ast::Style::Struct => {
				out.append(quote! {
					repr.push(c.text(" {"));
					repr.push(c.break_with(" ", Default::default()));
					repr.push(c.group(#name, Group::Inherit, { #fields }));
					repr.push(c.break_with(" ", Default::default()));
					repr.push(c.text("}"));
				});
			}

			ast::Style::Tuple | ast::Style::NewType => {
				out.append(quote! {
					repr.push(c.text("("));
					repr.push(c.group(#name, Group::Inherit, { #fields }));
					repr.push(c.text(")"));
				});
			}

			ast::Style::Unit =>
				(),
		}
	})
}

fn from_enum(name: Tokens, attr: &attr::Container, variants: &[ast::Variant]) -> Tokens {
	repr(|out| {
		// Print name
		out.append(if let Some(style) = attr.style {
			let style = from_style(style);

			quote! {
				repr.push(c.style(c.text(#name), #style));
			}
		}
		else {
			quote! {
				repr.push(c.text(#name));
			}
		});

		// Print variant name
		// Print body
	})
}

fn from_field(field: &ast::Field) -> Tokens {
	repr(|out| {
		// Print name, if present.
		if let Some(ref name) = field.ident {
			let name = name.to_string();

			out.append(if let Some(style) = field.attr.style {
				let style = from_style(style);

				quote! {
					repr.push(c.style(c.text(#name), #style));
				}
			}
			else {
				quote! {
					repr.push(c.text(#name));
				}
			});

			out.append(quote! {
				repr.push(c.text(": "));
			});
		}

		out.append(match field.attr.like {
			None => quote! {
				repr.push(c.text("none"));
			},

			Some(attr::Like::Unknown) => quote! {
				repr.push(c.text("unknown"));
			},

			Some(attr::Like::Debug) => quote! {
				repr.push(c.text("debug"));
			},

			Some(attr::Like::Iterator) => quote! {
				repr.push(c.text("iterator"));
			},
		});
	})
}

fn from_color(color: Color) -> Tokens {
	expr(|out| {
		match color {
			Color::Black =>
				out.append(quote! { Color::Black }),

			Color::Red =>
				out.append(quote! { Color::Red }),

			Color::Green =>
				out.append(quote! { Color::Green }),

			Color::Yellow =>
				out.append(quote! { Color::Yellow }),

			Color::Blue =>
				out.append(quote! { Color::Blue }),

			Color::Purple =>
				out.append(quote! { Color::Purple }),

			Color::Cyan =>
				out.append(quote! { Color::Cyan }),

			Color::White =>
				out.append(quote! { Color::White }),

			Color::Fixed(n) =>
				out.append(quote! { Color::Fixed(#n) }),

			Color::RGB(r, g, b) =>
				out.append(quote! { Color::RGB(#r, #g, #b) }),
		}
	})
}

fn from_style(style: Style) -> Tokens {
	block(|out| {
		out.append(quote! { let mut style = Style::new(); });

		if let Some(fg) = style.foreground {
			let color = from_color(fg);

			out.append(quote! {
				style = style.fg(#color);
			})
		}

		if let Some(bg) = style.background {
			let color = from_color(bg);

			out.append(quote! {
				style = style.on(#color);
			})
		}

		if style.is_bold {
			out.append(quote! {
				style = style.bold();
			});
		}

		if style.is_dimmed {
			out.append(quote! {
				style = style.dimmed();
			});
		}

		if style.is_italic {
			out.append(quote! {
				style = style.italic();
			});
		}

		if style.is_underline {
			out.append(quote! {
				style = style.underline();
			});
		}

		if style.is_blink {
			out.append(quote! {
				style = style.blink();
			});
		}

		if style.is_reverse {
			out.append(quote! {
				style = style.reverse();
			});
		}

		if style.is_hidden {
			out.append(quote! {
				style = style.hidden();
			});
		}

		if style.is_strikethrough {
			out.append(quote! {
				style = style.strikethrough();
			});
		}

		out.append(quote! { style });
	})
}
