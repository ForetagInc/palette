use std::{collections::HashMap, rc::Rc};
use yew::prelude::*;

use super::{Assets, Theme, Tokens};

#[derive(Properties, PartialEq)]
pub struct PaletteProviderProps {
	pub children: Children,
}

#[function_component]
pub fn PaletteProvider(props: &PaletteProviderProps) -> Html {
	let theme = use_memo(
		|_| Theme {
			name: "default",
			tokens: Tokens {
				colors: HashMap::from([("red", "#redcode"), ("yellow", "#redcode")]),
				spacing: HashMap::new(),
				breakpoints: HashMap::new(),
				media_queries: HashMap::new(),
			},
			assets: Assets,
		},
		(),
	);

	fn provide<T, A>(theme: Rc<Theme<T, A>>, props: &PaletteProviderProps) -> Html
	where
		T: Clone + PartialEq + 'static,
		A: Clone + PartialEq + 'static,
	{
		html! {
			<ContextProvider<Rc<Theme<T, A>>> context={theme}>
				{ for props.children.iter() }
			</ContextProvider<Rc<Theme<T, A>>>>
		}
	}

	provide(theme, props)
}

pub type Palette = Theme<Tokens, Assets>;
