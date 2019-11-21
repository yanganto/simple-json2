use simple_json::{ self, json::JsonValue };

fn main() {
  let json_str = r#"{"data":{"id":"bitcoin","rank":"1","symbol":"BTC","name":"Bitcoin","supply":"18057312.0000000000000000","maxSupply":"21000000.0000000000000000","marketCapUsd":"147569099750.7204918604574592","volumeUsd24Hr":"4580709179.8339913779178792","priceUsd":"8172.2628346190447316","changePercent24Hr":"0.2545279279476533","vwap24Hr":"8137.7203001478675222"},"timestamp":1574224303089}"#;

  let json_val: JsonValue = simple_json::parse_json(&json_str).unwrap();
  let data = json_val.get_object()[0].1.get_object();

  let mut sym = String::from("");
  let mut price = String::from("");
  data.iter()
    .filter(|(k, _)| "priceUsd" == k.iter().collect::<String>() || "symbol" == k.iter().collect::<String>())
    .for_each(|(k, v)| {
      let key = k.iter().collect::<String>();
      let val = v.get_string();
      if "priceUsd" == key { price = val.clone() } else { sym = val.clone() }
    });
  println!("{:?}, {:?}", sym, price);
}
