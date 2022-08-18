use yew::prelude::*;

use crate::{
	context::{Palette, PaletteProvider},
	Mix,
};

#[function_component]
pub fn App() -> Html {
	html! {
		<PaletteProvider>
			<Button disabled={true} />
		</PaletteProvider>
	}
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
				variants: Some(variants)
			}
		}
	};
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
	let theme = use_context::<Palette>().expect("No Palette context found");

	let classes = theme.mix(props, |t, _| {
		mix! {
			base: "bg:yellow",
			variants: {
				disabled: format!("bg:{} f:{}", t.color("yellow"), t.color("red")),
			}
		}
	});

	html! {
		<div class={classes}>
			{"button"}
		</div>
	}
}
