#![allow(dead_code)]

use simple_json2::{
	self, parse_json,
	json::{ JsonValue },
	impls::SimpleError
};

const ZERO_ASCII: u64 = 48;

fn main() -> Result<(), SimpleError> {
	parse_json_f64()?;
	parse_simple()?;
	Ok(())
}

fn parse_json_f64() -> Result<(), SimpleError> {
	let json_str = r#"{"USD":7073.33}"#;
	let json_val: JsonValue = parse_json(&json_str)?;
	let data = json_val.get_object()?[0].1.get_number_f64()?;
	assert_eq!(data, 7073.33);

	Ok(())
}

fn parse_simple() -> Result<(), SimpleError> {
	let json_str = r#"{"data":{"id":"bitcoin","symbol":"BTC"},"timestamp":1574224303089}"#;

	// To parse a string to JSON object

	let json: JsonValue = parse_json(&json_str)?;

	// To get a value out from the JSON object

	let json_obj = &json.get_object()?[0];

	// We use vec of char because in `no_std` env, there is no `String`
	assert_eq!(json_obj.0, "data".chars().collect::<Vec<char>>());

	let inner_obj = &json_obj.1.get_object()?[0];

	assert_eq!(inner_obj.0, "id".chars().collect::<Vec<char>>());
	assert_eq!(inner_obj.1.get_string()?, "bitcoin");

	Ok(())
}
