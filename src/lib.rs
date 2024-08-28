use base64::prelude::*;

pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes = hex::decode(hex_string).unwrap();
    return BASE64_STANDARD.encode(bytes);
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
}
