use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if let Some(rest) = encoded_value.strip_prefix('l') {
        if let Some((list, _)) = rest.split_once('e') {
            return serde_json::Value::Array(
                // TODO
                list.split(|c| c == 'l' || c == 'd' || c == 'e')
                    .map(|item| decode_bencoded_value(item))
                    .collect(),
            );
        }
    }

    if let Some(rest) = encoded_value.strip_prefix('i') {
        if let Some((digits, _)) = rest.split_once('e') {
            if let Ok(number) = digits.parse::<i64>() {
                return serde_json::Value::Number(number.into());
            }
        }
    }

    if let Some((len, rest)) = encoded_value.split_once(":") {
        if let Ok(len) = len.parse::<usize>() {
            return serde_json::Value::String(rest[..len].to_string());
        }
    }

    panic!("invalid encoded value: {}", encoded_value);
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

