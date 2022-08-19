use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Lit, LitStr, Token};

use crate::PeekValue;

pub struct MixNode {
	pub base: MixBase,
	// pub variants: HashMap<MixKey, MixValue>,
}

impl Parse for MixNode {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		MixBase::peek(input.cursor()).ok_or_else(|| input.error("Missing base Mix property"))?;

		let base = input.parse()?;

		Ok(MixNode { base })
	}
}

impl ToTokens for MixNode {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let base = &self.base.0;

		tokens.extend(quote! {
			let ba3 = #base;
		})
	}
}

pub struct MixBase(LitStr);

impl Parse for MixBase {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ident: Ident = input.parse()?;
		ident
			.to_string()
			.eq("base")
			.ok_or_else(|| input.error("Expected base identifier"))?;

		let _q: Token![:] = input.parse()?;

		Ok(Self(input.parse()?))
	}
}

impl PeekValue<()> for MixBase {
	fn peek(cursor: Cursor) -> Option<()> {
		let (ident, _) = cursor.ident()?;
		(ident == "base").as_option()
	}
}

pub type MixKey = Box<Ident>;

pub enum MixValue {
	Literal(Lit),
	Format(MixFormat),
}

pub struct MixFormat {
	pub lit: Lit,
	pub args: Vec<MixFormatArg>,
}

pub enum MixFormatArg {
	Literal(Lit),
	Expression(Expr),
}
