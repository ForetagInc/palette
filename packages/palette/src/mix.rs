use std::collections::HashMap;
use yew::prelude::Properties;

pub struct Mix<'a> {
	pub base: &'a str,
	pub variants: Option<HashMap<&'a str, String>>,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Tokens {
	pub colors: HashMap<&'static str, &'static str>,
}

impl Tokens {
	pub fn color(&self, color: &str) -> &str {
		self.colors.get(color).unwrap_or(&"")
	}
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Assets;
