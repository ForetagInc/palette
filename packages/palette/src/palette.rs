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
	pub name: String,
	pub tokens: T,
	pub assets: A,
}

impl<T, A> Theme<T, A>
where
	T: Clone + PartialEq + 'static,
	A: Clone + PartialEq + 'static,
{
	pub fn new(name: String, tokens: T, assets: A) -> Self {
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

		let mut classes: Vec<&str> = vec![];
		// Base
		classes.append(&mut mix.base.split(' ').into_iter().collect::<Vec<&str>>());

		Classes::from(classes)
	}
}
