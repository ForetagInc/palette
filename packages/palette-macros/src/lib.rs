mod mix;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::buffer::Cursor;
use syn::parse_macro_input;

use mix::MixNode;

trait PeekValue<T> {
	fn peek(cursor: Cursor) -> Option<T>;
}

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn mix(input: TokenStream) -> TokenStream {
	let root = parse_macro_input!(input as MixNode);
	TokenStream::from(root.into_token_stream())
}
