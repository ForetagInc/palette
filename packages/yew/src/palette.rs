use super::Mix;
use yew::Classes;

#[derive(Clone, PartialEq, Eq)]
pub struct Palette<T, A>
where
	T: Clone + PartialEq + 'static,
	A: Clone + PartialEq + 'static,
{
	pub default: Theme<T, A>,
	pub themes: Vec<Theme<T, A>>,
}

impl<T, A> Palette<T, A>
where
	T: Clone + PartialEq + 'static,
	A: Clone + PartialEq + 'static,
{
	pub fn new(themes: Vec<Theme<T, A>>) -> Self {
		Palette {
			default: themes.first().cloned().expect("No themes exist"),
			themes,
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Theme<T, A>
where
	T: Clone + PartialEq + 'static,
	A: Clone + PartialEq + 'static,
{
	pub name: &'static str,
	pub tokens: T,
	pub assets: A,
}

impl<T, A> Theme<T, A>
where
	T: Clone + PartialEq + 'static,
	A: Clone + PartialEq + 'static,
{
	pub fn new(name: &'static str, tokens: T, assets: A) -> Self {
		Self {
			name,
			tokens,
			assets,
		}
	}

	pub fn mix<P, Cb>(&self, props: &P, callback: Cb) -> Classes
	where
		Cb: Fn(T, A) -> Mix<'static>,
	{
		let theme = self.clone();
		let mix = callback(theme.tokens, theme.assets);

		let mut classes: Vec<String> = Vec::new();

		// Base
		classes.append(
			&mut mix
				.base
				.split(' ')
				.into_iter()
				.map(|x| x.to_string())
				.collect::<Vec<String>>(),
		);

		// Variants
		for (k, v) in mix.variants.unwrap() {
			classes.push(v);
		}

		Classes::from(classes)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use crate::{Assets, Theme, Tokens};

	#[test]
	fn create_theme() {
		let assets = Assets;

		let tokens = Tokens {
			colors: HashMap::new(),
			spacing: HashMap::new(),
			breakpoints: HashMap::new(),
			media_queries: HashMap::new(),
		};

		let theme = Theme::new("default", tokens, assets);

		assert_eq!(theme.name, "default")
	}
}
