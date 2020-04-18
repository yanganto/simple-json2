# simple-json2

Simple JSON parser written with Rust. Wasm / no_std ready.


```bash
# Build this project in `no_std` env:
just build

# Running test in `no_std` env:
just test

# Run this project in cli (a sample for parsing) in `std` env:
just run
```

## Usage

To parse a string into JSON object

```rust
use simple_json2::{ self, parse_json, impls::SimpleError };

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
	// If we are in `std` env, we can retrieve value back as `String`
	assert_eq!(inner_obj.1.get_string()?, "bitcoin");

	// -- snip
}

```

## Acknowledgement

This project is derived from [xlc/lite-json](https://github.com/xlc/lite-json/)
