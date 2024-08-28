use base64::prelude::*;

pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes = hex::decode(hex_string).unwrap();
    return BASE64_STANDARD.encode(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set1() {
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    }
}
