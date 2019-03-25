mod set1;

use std::io::{Error, ErrorKind};

pub fn base64_encode(bytes: &[u8]) -> String {
    let table = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    // 当输入的长度非3的倍数时,余1会padding两个等号,余2时会padding一个等号
    let padding_len = ((-(bytes.len() as isize % 3) + 3) % 3) as usize;
    // base64实际将字节流以3字节分组,装换为4字节分组的编码后的字符流
    let mut result = String::with_capacity((bytes.len() + 3 - 1) / 3 * 4);

    let iter_group = bytes.len() / 3;
    let mut idx = 0;
    let mut code: u8;
    for _ in 0..iter_group {
        code = bytes[idx] >> 2;
        result.push(table[code as usize]);

        code = (bytes[idx] << 4) & 0b0011_0000 | (bytes[idx + 1] >> 4);
        result.push(table[code as usize]);

        code = (bytes[idx + 1] << 2) & 0b0011_1100 | (bytes[idx + 2] >> 6);
        result.push(table[code as usize]);

        code = bytes[idx + 2] & 0b0011_1111;
        result.push(table[code as usize]);

        idx += 3;
    }

    if padding_len == 1 {
        code = bytes[idx] >> 2;
        result.push(table[code as usize]);

        code = (bytes[idx] << 4) & 0b0011_0000 | (bytes[idx + 1] >> 4);
        result.push(table[code as usize]);

        code = (bytes[idx + 1] << 2) & 0b0011_1100;
        result.push(table[code as usize]);

        result.push_str("=");
    } else if padding_len == 2 {
        code = bytes[idx] >> 2;
        result.push(table[code as usize]);

        code = (bytes[idx] << 4) & 0b0011_0000;
        result.push(table[code as usize]);

        result.push_str("==");
    }
    
    result
}

pub fn hex_to_u8(hex: &str) -> Result<u8, Error> {
    if let Ok(t) = u8::from_str_radix(hex, 16) {
        return Ok(t);
    }

    return Err(Error::new(ErrorKind::InvalidInput, "Non-Hex Data"));
}

pub fn repeat_xor_encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let key_size = key.len();
    for (i, each) in plain.iter().enumerate() {
        result.push(each ^ key[i % key_size]);
    }

    result
}

// 计算两个字节的汉明距离
pub fn hamming_distant(a: u8, b: u8) -> u8 {
    let mut x = a ^ b;
    let mut distant = 0u8;

    while x > 0 {
        distant += x & 1;
        x >>= 1;
    }

    distant
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
    fn test_repeat_xor_encrypt() {
        let result = repeat_xor_encrypt("0123".as_bytes(), "ABC".as_bytes());
        assert_eq!("qsqr", String::from_utf8(result).unwrap());
    }

    #[test]
    fn test_hamming_distant() {
        assert_eq!(hamming_distant(0x01, 0x07), 2);
        assert_eq!(hamming_distant(5, 5), 0);
        assert_eq!(hamming_distant(0xff, 0x00), 8);
    }
}

