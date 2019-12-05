use simple_json::{ self, json::JsonValue };

fn main() {
  let json_str = r#"{"data":{"id":"bitcoin","rank":"1","symbol":"BTC","name":"Bitcoin","supply":"18057312.0000000000000000","maxSupply":"21000000.0000000000000000","marketCapUsd":"147569099750.7204918604574592","volumeUsd24Hr":"4580709179.8339913779178792","priceUsd":"8172.2628346190447316","changePercent24Hr":"0.2545279279476533","vwap24Hr":"8137.7203001478675222"},"timestamp":1574224303089}"#;

  let json_val: JsonValue = simple_json::parse_json(&json_str).unwrap();
  let data = json_val.get_object()[0].1.get_object();

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
      let val = v.get_bytes();
      match k_bytes {
        k_bytes if k_bytes == price_bytes => {
          let dot_pos = val.iter().position(|&i| i == ('.' as u8)).unwrap();
          dollars = val.get(0..(dot_pos - 1)).unwrap().to_vec();
          cents = val.get((dot_pos + 1)..).unwrap().to_vec();
        },
        k_bytes if k_bytes == sym_bytes => { sym = val.clone(); },
        _ => { panic!("Unexpected type"); },
      };
    });
  println!("{:?}, {:?}, {:?}",
    String::from_utf8(sym).unwrap(),
    String::from_utf8(dollars).unwrap(),
    String::from_utf8(cents).unwrap(),
  );
}
