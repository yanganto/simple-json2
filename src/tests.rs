extern crate alloc;
use alloc::vec;
use crate::{
	parse_json,
	json::{JsonValue, NumberValue}
};

#[test]
fn simple_test() {
	assert_eq!(
		parse_json(&r#"{"USD":7073.33}"#)
			.map_err(|_| ()),
		Ok(JsonValue::Object(vec![(
			vec!['U', 'S', 'D'], JsonValue::Number(NumberValue {
				integer: 7073, fraction: 33, fraction_length: 2, exponent: 0
			})
		)]))
	);
}

#[test]
fn empty_array_test() {
	assert_eq!(
		parse_json(&r#"{"E": []}"#)
			.map_err(|_| ()),
		Ok(JsonValue::Object(vec![(
			vec!['E'], JsonValue::Array(vec![])
		)]))
	);
}

#[test]
fn thorough_test() {
	assert_eq!(
		parse_json(&r#"{
			"test1": 1,
			"test2": [1e-4, 2.041e2, true, false, null, "\"1n\""],
			"test3": {
				"test4": [ { "test5": 5 }, { "test6": 6 } ]
			}
		}"#)
			.map_err(|_| ()),
		Ok(JsonValue::Object(vec![
			(vec!['t', 'e', 's', 't', '1'], JsonValue::Number(NumberValue {
				integer: 1,
				fraction: 0,
				fraction_length: 0,
				exponent: 0
			})), (
				vec!['t', 'e', 's', 't', '2'], JsonValue::Array(vec![
					JsonValue::Number(NumberValue {
						integer: 1,
						fraction: 0,
						fraction_length: 0,
						exponent: -4,
					}),
					JsonValue::Number(NumberValue {
						integer: 2,
						fraction: 41,
						fraction_length: 3,
						exponent: 2,
					}),
					JsonValue::Boolean(true),
					JsonValue::Boolean(false),
					JsonValue::Null,
					JsonValue::String(vec!['\"', '1', 'n', '\"'])
				])
			), (
				vec!['t', 'e', 's', 't', '3'], JsonValue::Object(vec![
					(vec!['t', 'e', 's', 't', '4'], JsonValue::Array(vec![
						JsonValue::Object(vec![(
							vec!['t','e','s','t','5'],
							JsonValue::Number(NumberValue { integer: 5, fraction: 0, fraction_length: 0, exponent: 0 })
						)]),
						JsonValue::Object(vec![(
							vec!['t','e','s','t','6'],
							JsonValue::Number(NumberValue { integer: 6, fraction: 0, fraction_length: 0, exponent: 0 })
						)]),
					]))
				])
			)
		]))
	)
}
