#[derive(Clone, PartialEq, Eq)]
pub struct Palette<T, A> {
	pub default: Theme<T, A>,
	pub themes: Vec<Theme<T, A>>,
}

impl<T: Clone, A: Clone> Palette<T, A> {
	pub fn new(themes: Vec<Theme<T, A>>) -> Self {
		Palette {
			default: themes.first().cloned().expect("No themes exist"),
			themes,
		}
	}

	pub fn mix() -> String {
		String::from("")
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Theme<T, A> {
	pub name: String,
	pub tokens: T,
	pub assets: A,
}

impl<T, A> Theme<T, A> {
	pub fn new(name: String, tokens: T, assets: A) -> Self {
		Self {
			name,
			tokens,
			assets,
		}
	}
}

#[derive(Default)]
pub struct Token {
	key: String,
	value: String,
}

#[derive(Default)]
pub struct Tokens {
	colors: Vec<Token>,
}
