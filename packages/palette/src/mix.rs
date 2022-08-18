use std::collections::HashMap;
use yew::prelude::Properties;

pub struct Mix<'a> {
	pub base: &'a str,
	pub variants: HashMap<&'a str, String>,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Tokens {
	pub colors: HashMap<&'static str, &'static str>,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Assets;
