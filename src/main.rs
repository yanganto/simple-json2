#![allow(dead_code)]

use simple_json::{ self, json, json::JsonValue, json::JsonObject, impls::SimpleError };
use simple_json::parser::Error;

const ZERO_ASCII: u64 = 48;

fn main() -> Result<(), SimpleError> {
	parse_json_f64()?;
	coincap()?;
	coinmarketcap()?;
	Ok(())
}

fn parse_json_f64() -> Result<(), SimpleError> {
	let json_str = r#"{"USD":7073.33}"#;
	let json_val: JsonValue = simple_json::parse_json(&json_str)?;
	let data = json_val.get_object()?[0].1.get_number_f64()?;
	println!("{:?}", data);
	Ok(())
}

fn coincap() -> Result<(), SimpleError> {
	let json_str = r#"{"data":{"id":"bitcoin","rank":"1","symbol":"BTC","name":"Bitcoin","supply":"18057312.0000000000000000","maxSupply":"21000000.0000000000000000","marketCapUsd":"147569099750.7204918604574592","volumeUsd24Hr":"4580709179.8339913779178792","priceUsd":"8172.2628346190447316","changePercent24Hr":"0.2545279279476533","vwap24Hr":"8137.7203001478675222"},"timestamp":1574224303089}"#;

	let json_val: JsonValue = simple_json::parse_json(&json_str)?;
	let data = json_val.get_object()?[0].1.get_object()?;

	let mut sym: Vec<u8> = Vec::new();
	let mut dollars: Vec<u8> = Vec::new();
	let mut cents: Vec<u8> = Vec::new();
	let price_bytes = "priceUsd".as_bytes().to_vec();
	let sym_bytes = "symbol".as_bytes().to_vec();

	data.iter()
		.filter(|(k, _)| {
			let k_bytes = k.iter().map(|c| *c as u8).collect::<Vec<_>>();
			k_bytes == price_bytes || k_bytes == sym_bytes
		})
		.for_each(|(k, v)| {
			let k_bytes = k.iter().map(|c| *c as u8).collect::<Vec<_>>();
			let val = v.get_bytes().unwrap();
			match k_bytes {
				k_bytes if k_bytes == price_bytes => {
					let dot_pos = val.iter().position(|&i| i == ('.' as u8)).unwrap();
					dollars = val.get(0..dot_pos).unwrap().to_vec();
					cents = val.get((dot_pos + 1)..).unwrap().to_vec();
				},
				k_bytes if k_bytes == sym_bytes => { sym = val.clone(); },
				_ => { SimpleError::plain_str("Unexpected type"); },
			};
		});

	println!("{:?}, {:?}, {:?}",
		String::from_utf8(sym).unwrap(),
		String::from_utf8(dollars).unwrap(),
		String::from_utf8(cents).unwrap(),
	);
	Ok(())
}

fn coinmarketcap() -> Result<(), SimpleError> {
	let json_str = r#"{
		"status": {
			"timestamp": "2019-12-05T14:57:06.935Z",
			"error_code": 0,
			"error_message": null,
			"elapsed": 8,
			"credit_count": 1 },
		"data": {
			"BTC": {
				"id": 1,
				"name": "Bitcoin",
				"symbol": "BTC",
				"slug": "bitcoin",
				"num_market_pairs": 7919,
				"date_added": "2013-04-28T00:00:00.000Z",
				"tags": [
					"mineable"
				],
				"max_supply": 21000000,
				"circulating_supply": 17906012,
				"total_supply": 17906012,
				"platform": null,
				"cmc_rank": 1,
				"last_updated": "2019-08-30T18:51:28.000Z",
				"quote": {
					"USD": {
						"price": 9558.55165723,
						"volume_24h": 13728947008.2722,
						"percent_change_1h": -0.127291,
						"percent_change_24h": 0.328918,
						"percent_change_7d": -8.00576,
						"market_cap": 171155540318.86005,
						"last_updated": "2019-08-30T18:51:28.000Z"
					}
				}
			}}}"#;

	let json_val: JsonValue = simple_json::parse_json(&json_str)?;
	let btc_data: &JsonObject = json_val.get_object()?[1]
		.1.get_object()?[0]
		.1.get_object()?;

	let quote_usd_data: &JsonObject = btc_data.iter()
		.filter_map(|(k, v)|
			if vecchars_to_vecbytes(k) == str_to_vecbytes("quote") { Some(v) } else { None }
		)
		.nth(0).ok_or(SimpleError::plain_str("option::NoneError"))?
		.get_object()?[0]
		.1.get_object()?;

	// println!("{:?}", quote_usd_data);
	let price: &JsonValue = quote_usd_data.iter()
		.filter_map(|(k, v)|
			if vecchars_to_vecbytes(k) == str_to_vecbytes("price") { Some(v) } else { None }
		)
		.nth(0).ok_or(SimpleError::plain_str("option::NoneError"))?;

	// print!("{:?}", price);
	if let JsonValue::Number(json::NumberValue{integer: dollars, fraction: cents, ..}) = price {
		println!("{:?}, {:?}", dollars, cents);
		println!("{:?}, {:?}", dollars, vecbytes_to_u64(u64_to_vecbytes(*cents),4));
	}

	Ok(())
}

fn vecchars_to_vecbytes<I: IntoIterator<Item = char> + Clone>(it: &I) -> Vec<u8> {
	it.clone().into_iter().map(|c| c as u8).collect::<_>()
}

fn str_to_vecbytes<'a>(str_val: &'a str) -> Vec<u8> {
	str_val.as_bytes().to_vec()
}

fn u64_to_vecbytes(num: u64) -> Vec<u8> {
	let mut vecbytes: Vec<u8> = vec![];
	let mut num = num;
	while num > 0 {
		let digit = num % 10;
		num /= 10;
		vecbytes.push((digit + ZERO_ASCII) as u8);
	}
	vecbytes.into_iter().rev().collect::<Vec<_>>()
}

fn vecbytes_to_u64(it: impl IntoIterator<Item = u8>, take: usize) -> u64 {
	it.into_iter().enumerate()
		.filter_map(|(pos, byte)| {
			// pos need to take an additional digit for rounding
			if take == 0 || pos <= take { return Some((pos, (byte as char).to_digit(10).unwrap())); }
			None
		})
		.fold(0, |acc, (pos, digit)| {
			if take == 0 || pos < take { acc * 10 + digit as u64 }
			// rounding
			else { if digit >= 5 { acc + 1 } else { acc } }
		})
}
