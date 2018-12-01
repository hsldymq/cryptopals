use std::cmp;

// challenge 1
pub fn hex_to_base64(hex: &str) -> String {
    let len = hex.len();
    let mut bytes: Vec<u8> = Vec::with_capacity((len + 1) / 2);
    let mut from = 0;
    let mut to = cmp::min(len, 2);

    while from < len {
        bytes.push(u8::from_str_radix(&hex[from..to], 16).unwrap());

        from = to;
        to = cmp::min(len, to + 2);
    }

    base64_encode(&bytes)
}

pub fn base64_encode(bytes: &[u8]) -> String {
    let table = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];
    let padding = ["", "", "==", "", "=", ""];

    let mut hold = 0u8; 
    let mut offset = 2; 
    let mut code: u8;

    let bit_len = bytes.len() * 8;
    let mut result = String::with_capacity((bit_len + 5) / 6 + padding[bit_len % 6].len());

    for b in bytes {
        code = hold | (b >> offset);
        hold = (b << (6 - offset)) & 0b0011_1111u8;
        offset += 2;
        result.push(table[code as usize]);

        if offset == 8 {
            result.push(table[hold as usize]);
            hold = 0;
            offset = 2;
        }
    }
    if hold > 0 {
        result.push(table[hold as usize]);
    }

    result.push_str(padding[bit_len % 6]);
    
    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("".as_bytes()), "");
        assert_eq!(base64_encode("f".as_bytes()), "Zg==");
        assert_eq!(base64_encode("fo".as_bytes()), "Zm8=");
        assert_eq!(base64_encode("foo".as_bytes()), "Zm9v");
        assert_eq!(base64_encode("foob".as_bytes()), "Zm9vYg==");
        assert_eq!(base64_encode("fooba".as_bytes()), "Zm9vYmE=");
        assert_eq!(base64_encode("foobar".as_bytes()), "Zm9vYmFy");
    }

    #[test]
    fn test_hex_to_base64() {
        let base64_value = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(base64_value, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
    
}