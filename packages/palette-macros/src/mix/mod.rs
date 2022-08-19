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
		let mut base: Option<MixBase> = None;

		while let Ok(el_type) = input.parse() {
			match el_type {
				MixElementType::Base => {
					base.is_none()
						.ok_or_else(|| input.error("Property `base` already assigned"))?;

					base = Some(input.parse()?)
				}
			}

			let _comma: Token![,] = input.parse()?;
		}

		Ok(MixNode {
			base: base.ok_or_else(|| input.error("Property `base` needed to build Mix"))?,
		})
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
		input
			.parse::<Ident>()?
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

pub enum MixElementType {
	Base,
}

impl Parse for MixElementType {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if MixBase::peek(input.cursor()).is_some() {
			Ok(MixElementType::Base)
		} else {
			Err(input.error(format!(
				"Unexpected token: `{}`",
				input.parse::<Ident>()?.to_string()
			)))
		}
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
