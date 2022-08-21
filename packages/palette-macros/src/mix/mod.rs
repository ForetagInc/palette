use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Error, Expr, Ident, Lit, LitStr, Token};

use proc_macro2::{Span, TokenStream};
use std::collections::HashMap;
use syn::__private::str;
use syn::spanned::Spanned;

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
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let base = &self.base.0;
		let variants = &self.variants;

		let mut stream = TokenStream::new();

		stream.extend(quote! {
			let base = #base;
			let mut variants = None;
		});

		if let Some(v) = variants {
			stream.extend(quote! {
				variants = Some(#v);
			})
		}

		stream.extend(quote! {
			Mix {
				base,
				variants
			}
		});

		tokens.extend(quote! {
			{
				#stream
			}
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
		let _ = braced!(content in input);

		let mut keys = Vec::new();

		while let ident = content.parse::<Ident>()? {
			let _q: Token![:] = content.parse()?;

			let val = content.parse::<MixValue>()?;
			let val_span = val.span();

			let ident_str = ident.to_string();

			(!keys.contains(&ident.to_string()))
				.ok_or(Error::new(ident.span(), "Already declared variant"))?;

			map.insert(ident, val);
			keys.push(ident_str);

			content
				.cursor()
				.ident()
				.is_none()
				.ok_or(Error::new(val_span, "Expected `,`"))?;

			if content.cursor().eof() {
				break;
			}

			content.parse::<Token![,]>()?;
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

impl ToTokens for MixVariants {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let mut stream = TokenStream::new();

		stream.extend(quote! {
			let mut variants = std::collections::HashMap::new();
		});

		for (k, v) in &self.0 {
			stream.extend(quote! {
				variants.insert(stringify!(#k), String::from(#v));
			});
		}

		stream.extend(quote! {
			variants
		});

		tokens.extend(quote! {
			{
				#stream
			}
		})
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

impl ToTokens for MixValue {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			MixValue::Literal(l) => l.to_tokens(tokens),
		}
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
