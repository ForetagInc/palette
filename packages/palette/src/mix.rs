use std::collections::HashMap;
use yew::prelude::Properties;

pub struct Mix<'a> {
	pub base: &'a str,
	pub variants: Option<HashMap<&'a str, String>>,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Tokens {
	pub colors: HashMap<&'static str, &'static str>,
	pub spacing: HashMap<&'static str, i32>,
}

impl Tokens {
	pub fn color(&self, color: &str) -> &str {
		self.colors.get(color).unwrap_or(&"")
	}

	pub fn spacing(&self, spacing: &str) -> &i32 {
		self.spacing.get(spacing).unwrap_or(&0)
	}
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Assets;
