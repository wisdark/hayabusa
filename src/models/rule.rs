extern crate serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub severity: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Toml {
    pub rule: Rule,
}