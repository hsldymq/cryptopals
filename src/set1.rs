use std::cmp;
use std::io::{Error, ErrorKind};
use super::*;

// challenge 1
pub fn hex_to_base64(hex: &str) -> Result<String, Error> {
    let len = hex.len();
    let mut bytes: Vec<u8> = Vec::with_capacity((len + 1) / 2);
    let mut from = 0;
    let mut to = cmp::min(len, 2);

    while from < len {
        if let Ok(t) = u8::from_str_radix(&hex[from..to], 16) {
            bytes.push(t);
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Non-Hex Data"));
        }
        

        from = to;
        to = cmp::min(len, to + 2);
    }

    Ok(base64_encode(&bytes))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let base64_value = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        assert_eq!(base64_value, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
    
}