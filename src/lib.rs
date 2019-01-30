mod set1;

use std::io::{Error, ErrorKind};

pub fn base64_encode(bytes: &[u8]) -> String {
    let table = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    let mut code: u8;
    let bit_len = bytes.len() * 8;
    // bit_len始终是8的倍数,所以bit_len % 6只会有三种可能的值,0, 2, 4
    // 其中0不需要padding, 2需要padding两个=, 4需要padding一个=
    // 这里做了一个变换,将bit_len % 6的结果由4变为1,其他两个只保持不变,这样就正好跟padding的长度保持一致了.
    let padding_len = ((-(bit_len as isize % 6 / 2) + 3) % 3) as usize;
    let mut result = String::with_capacity(bit_len / 6 + padding_len);

    let iter_group = bit_len / 24;
    let mut idx = 0;
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

