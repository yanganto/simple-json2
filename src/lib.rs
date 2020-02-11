#![cfg_attr(not(feature = "std"), no_std)]

use crate::impls::SimpleError;
use crate::json::{Json, JsonValue};
use crate::parser::Parser;

pub mod impls;
pub mod json;
pub mod parser;

#[cfg(test)]
mod tests;

pub fn parse_json(input: &str) -> Result<JsonValue, SimpleError> {
  Json::parse(&input, Default::default()).map(|(ret, _)| ret)
}
