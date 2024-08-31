use std::collections::HashMap;
use base64::prelude::*;

pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes = hex::decode(hex_string).unwrap();
    BASE64_STANDARD.encode(bytes)
}

pub fn xor(hex_string1: &str, hex_string2: &str) -> String {
    let b1 = hex::decode(hex_string1).unwrap();
    let b2 = hex::decode(hex_string2).unwrap();
    hex::encode(
        b1.iter()
            .zip(b2.iter())
            .map(|(x, y)| x ^ y)
            .collect::<Vec<u8>>(),
    )
}

fn score_string(hex_string: &str) -> i32 {
    let ranking: HashMap<char, i32> = HashMap::from([
        ('e', 13),
        ('t', 12),
        ('a', 11),
        ('o', 10),
        ('i', 9),
        ('n', 8),
        (' ', 7),
        ('s', 6),
        ('h', 5),
        ('r', 4),
        ('d', 3),
        ('l', 2),
        ('u', 1),
    ]);
    hex_string.chars().map(|c| {
        match ranking.get(&c.to_ascii_lowercase()) {
            Some(&value) => value,
            None => 0
        }
    }).collect::<Vec<i32>>().iter().sum()
}

fn xor_cipher(hex_string: &str) -> (i32, u8, String){
    let bytes = hex::decode(hex_string).unwrap();
    let mut out = (0, 0u8, String::new());
    for byte in 0u8..=255 {
        let decrypted = bytes.iter().map(|x| x ^ byte).collect::<Vec<u8>>();
        match String::from_utf8(decrypted) {
            Ok(decrypted_string) => {
                let score = score_string(&decrypted_string);
                if score > out.0 {
                    out = (score, byte, decrypted_string.to_string());
                }
                // println!("{}, {}: {}", byte, score_string(&decrypted_string),decrypted_string)
            },
            // If we could not decrypt, don't care
            Err(_) => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set1ch1() {
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    }

    #[test]
    fn set1ch2() {
        assert_eq!(
            xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        )
    }

    #[test]
    fn set1ch3() {
        let output = xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        assert_eq!(output.1, 88);
        assert_eq!(output.2, "Cooking MC's like a pound of bacon")
    }
}
