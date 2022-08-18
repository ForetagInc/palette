use std::rc::Rc;
use yew::prelude::*;

use crate::{Assets, Mix, Theme, Tokens};

#[function_component]
pub fn App() -> Html {
	let theme = use_memo(
		|_| Theme {
			name: String::from("Default"),
			tokens: Tokens {
				colors: vec![("red", "#redcode"), ("yellow", "#redcode")]
					.iter()
					.copied()
					.collect(),
			},
			assets: Assets,
		},
		(),
	);

	fn get_html<T, A>(theme: Rc<Theme<T, A>>) -> Html
	where
		T: Clone + PartialEq + 'static,
		A: Clone + PartialEq + 'static,
	{
		html! {
			<ContextProvider<Rc<Theme<T, A>>> context={theme}>
				<Button />
			</ContextProvider<Rc<Theme<T, A>>>>
		}
	}

	get_html(theme)
}

#[derive(Properties, PartialEq, Eq)]
pub struct ButtonProps {
	#[prop_or(false)]
	disabled: bool,

	#[prop_or(false)]
	bold: bool,
}

macro_rules! mix {
	{
		base: $base:expr,
		variants: {
			$(
				$prop:ident : $value:expr $(,)?
			)*
		}
	} => {
		{
			let mut variants = std::collections::HashMap::new();

			$(
				variants.insert(stringify!($prop), $value);
			)*

			Mix {
				base: stringify!($base),
				variants
			}
		}
	};
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
	let theme = use_context::<Theme<Tokens, Assets>>().expect("No Palette context found");

	let classes = theme.mix(props, |t, _| {
		mix! {
			base: "bg:yellow",
			variants: {
				disabled: format!("bg:{} f:{}", t.colors.get("yellow").unwrap_or(&""), t.colors.get("red").unwrap_or(&"")),
			}
		}
	});

	html! {
		<div class={classes}>
			{"button"}
		</div>
	}
}
