use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Expr, Ident, Lit, LitStr, Token};

use std::collections::HashMap;

use crate::PeekValue;

pub struct MixNode {
	pub base: MixBase,
	pub variants: Option<MixVariants>,
}

impl Parse for MixNode {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut base = None;
		let mut variants = None;

		while let Ok(el_type) = input.parse() {
			match el_type {
				MixElementType::Base => {
					base.is_none()
						.ok_or_else(|| input.error("Property `base` already assigned"))?;

					base = Some(input.parse()?)
				}
				MixElementType::Variants => {
					variants
						.is_none()
						.ok_or_else(|| input.error("Property `variants` already assigned"))?;

					variants = Some(input.parse()?)
				}
			}

			input.parse::<Token![,]>().ok();
		}

		Ok(MixNode {
			base: base.ok_or_else(|| input.error("Property `base` needed to build Mix"))?,
			variants,
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

pub struct MixVariants(HashMap<MixKey, MixValue>);

impl Parse for MixVariants {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input
			.parse::<Ident>()?
			.to_string()
			.eq("variants")
			.ok_or_else(|| input.error("Expected variants identifier"))?;

		let _q: Token![:] = input.parse()?;

		input
			.cursor()
			.group(proc_macro2::Delimiter::Brace)
			.is_some()
			.ok_or_else(|| input.error("Block expected"))?;

		let mut map = HashMap::new();

		let content;
		let brace = braced!(content in input);

		while let ident = content.parse::<Ident>()? {
			let _q: Token![:] = content.parse()?;

			let val = content.parse()?;

			map.insert(ident, val);

			if content.parse::<Token![,]>().is_err() {
				break;
			}
		}

		Ok(Self(map))
	}
}

impl PeekValue<()> for MixVariants {
	fn peek(cursor: Cursor) -> Option<()> {
		let (ident, _) = cursor.ident()?;
		(ident == "variants").as_option()
	}
}

pub enum MixElementType {
	Base,
	Variants,
}

impl Parse for MixElementType {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if MixBase::peek(input.cursor()).is_some() {
			Ok(MixElementType::Base)
		} else if MixVariants::peek(input.cursor()).is_some() {
			Ok(MixElementType::Variants)
		} else {
			Err(input.error(format!(
				"Unexpected token: `{}`",
				input.parse::<Ident>()?.to_string()
			)))
		}
	}
}

pub type MixKey = Ident;

pub enum MixValue {
	Literal(LitStr),
	// Format(MixFormat),
}

impl Parse for MixValue {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self::Literal(input.parse()?))
	}
}

pub struct MixFormat {
	pub lit: LitStr,
	pub args: Vec<MixFormatArg>,
}

pub enum MixFormatArg {
	Literal(LitStr),
	Expression(Expr),
}
