use palette_macros::mix;
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

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
	let theme = use_context::<Palette>().expect("No Palette context found");

	let classes = theme.mix(props, |tokens, _| {
		mix! {
			base: "bg:yellow",
			variants: {
				disabled: "bg:{} f:{} my:{}" => (
					tokens.color("yellow"),
					tokens.color("red"),
					tokens.spacing("8")
				),
				bold: "f:bold"
			}
		}
	});

	html! {
		<div class={classes}>
			{"button"}
		</div>
	}
}
