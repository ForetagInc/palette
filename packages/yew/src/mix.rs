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
	pub breakpoints: HashMap<&'static str, &'static str>,
	pub media_queries: HashMap<&'static str, &'static str>,
}

impl Tokens {
	pub fn color(&self, color: &str) -> &str {
		self.colors.get(color).unwrap_or(&"")
	}

	pub fn spacing(&self, spacing: &str) -> &i32 {
		self.spacing.get(spacing).unwrap_or(&0)
	}

	pub fn breakpoints(&self, breakpoint: &str) -> &str {
		self.breakpoints.get(breakpoint).unwrap_or(&"")
	}

	pub fn media_queries(&self, query: &str) -> &str {
		self.media_queries.get(query).unwrap_or(&"")
	}
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Assets;
