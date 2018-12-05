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
        v ^= hex_to_u8(&a[i..(i+1)]).unwrap();
        v ^= hex_to_u8(&b[i..(i+1)]).unwrap();

        result.push(table[v as usize]);
    }

    Ok(result)
}

// challenge 3
// Key: 0x58(X)
// Plain Text: Cooking MC's like a pound of bacon 
// BTW: Cooking MC's like a pound of bacon是Vanilla Ice在80年代的一首说唱歌曲Ice Ice Baby里的歌词
pub fn single_byte_xor_cipher() {
    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let len = encrypted.len();

    for i in 0..128 {
        let mut from = 0;
        let mut to = 2;
        let mut s = format!("Key: 0x{:x} - ", i);
        
        while from < len {
            let c = hex_to_u8(&encrypted[from..to]).unwrap();
            s.push_str(&format!("{}", (c ^ i as u8) as char));

            from = to;
            to = cmp::min(to + 2, len);
        }
        println!("{}", s);
    }
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

    #[test]
    fn test_single_byte_xor_cipher() {
        single_byte_xor_cipher();
    }
}