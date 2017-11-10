use quote::{Tokens, ToTokens};
use ast::Container;

impl<'a> ToTokens for Container<'a> {
	fn to_tokens(&self, out: &mut Tokens) {
		out.append("Rc::new(Document::Empty)");
	}
}
