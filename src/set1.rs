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

// challenge 2
pub fn fixed_xor(a: &str, b: &str) -> Result<String, Error> {
    if a.len() != b.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Length Not Equal"));
    }

    let table = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    let mut result = String::with_capacity(a.len());

    for i in 0..a.len() {
        let mut v = 0u8;
        if let Ok(t) = u8::from_str_radix(&a[i..(i+1)], 16) {
            v ^= t;
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Non-Hex Data"));
        }
        if let Ok(t) = u8::from_str_radix(&b[i..(i+1)], 16) {
            v ^= t;
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Non-Hex Data"));
        }

        result.push(table[v as usize]);
    }

    Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let base64_value = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        assert_eq!(base64_value, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
    
    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        assert_eq!(fixed_xor(a, b).unwrap(), "746865206b696420646f6e277420706c6179");
    }
}