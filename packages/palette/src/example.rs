use crate::{Palette, Theme};

use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone)]
pub struct Tokens;

impl Tokens {
	fn new() -> Self {
		Tokens
	}
}

#[derive(Clone)]
pub struct Assets;

impl Assets {
	fn new() -> Self {
		Assets
	}
}

#[function_component]
pub fn PaletteProvider<T, A>() -> Html {
	let palette = use_memo(
		|_| Palette::new(vec![Theme::new(String::from("default"), Tokens, Assets)]),
		(),
	);

	html! {
		<ContextProvider<Rc<Palette<T, A>>> context={palette}>
			<Button />
		</ContextProvider<Rc<Palette<T, A>>>>
	}
}

#[function_component]
pub fn Button() -> Html {
	html! {
		<button>
			{"button"}
		</button>
	}
}
