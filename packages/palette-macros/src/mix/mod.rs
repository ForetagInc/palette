use boolinator::Boolinator;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parenthesized, Error, Expr, Ident, LitStr, Token};

use std::collections::HashMap;
use syn::spanned::Spanned;

use crate::PeekValue;

pub struct MixNode {
	pub base: MixBase,
	pub variants: Option<MixVariants>,
}

impl Parse for MixNode {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let props = input.parse::<Ident>()?;

		let _: Token![=>] = input.parse()?;

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

					let mut v = input.parse::<MixVariants>()?;
					v.props = Some(props.clone());

					variants = Some(v)
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

pub struct MixVariants {
	props: Option<Ident>,
	map: HashMap<MixKey, MixValue>,
}

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

			let ident_str = ident.to_string();

			(!keys.contains(&ident.to_string()))
				.ok_or(Error::new(ident.span(), "Already declared variant"))?;

			map.insert(ident, val);
			keys.push(ident_str);

			if content.cursor().eof() {
				break;
			}

			content.parse::<Token![,]>()?;
		}

		Ok(Self { props: None, map })
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

		if let Some(props) = &self.props {
			for (k, v) in &self.map {
				// For type-checking whether the variant key is present on the component props struct
				// We just add struct.{variant} so that it will throw a compiler error when it's not
				// present on the struct.

				stream.extend(quote! {
					#props.#k;

					variants.insert(stringify!(#k), #v);
				});
			}
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
	Format(MixFormat),
}

impl Parse for MixValue {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input
			.parse::<LitStr>()
			.map(|s| Self::Literal(s))
			.or(input.parse::<MixFormat>().map(|f| Self::Format(f)))
	}
}

impl ToTokens for MixValue {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			MixValue::Literal(l) => tokens.extend(quote! {
				String::from(#l)
			}),
			MixValue::Format(f) => f.to_tokens(tokens),
		}
	}
}

pub struct MixFormat {
	pub lit: LitStr,
	pub args: Vec<Expr>,
}

impl Parse for MixFormat {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		(input.parse::<Ident>()?.to_string() == "format")
			.ok_or(input.error("`format` identifier expected"))?;

		let _ex: Token![!] = input.parse()?;

		input
			.cursor()
			.group(proc_macro2::Delimiter::Parenthesis)
			.is_some()
			.ok_or(input.error("Parenthesis expected"))?;

		let content;
		let _ = parenthesized!(content in input);

		let lit = content.parse::<LitStr>()?;

		let _c: Token![,] = content.parse()?;

		let mut args = Vec::new();

		while let expr = content.parse::<Expr>()? {
			args.push(expr);

			if content.cursor().eof() {
				break;
			}

			content.parse::<Token![,]>()?;
		}

		Ok(Self { lit, args })
	}
}

impl ToTokens for MixFormat {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let lit = &self.lit;
		let args = &self.args;

		tokens.extend(quote! {
			format!(#lit, #(#args),*)
		})
	}
}
