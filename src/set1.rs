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

// challenge 3
// 密钥是'X'
// 明文是"Cooking MC's like a pound of bacon"
// 思路:
// 首先,不能直接按照"ETAOIN SHRDLU"的字母分布将出现频率最高的字符假设为E来异或得到密钥,试过,无效.
// 但是我也不可能一个一个去试.但是,当我数一数字符数量发现有34个,你会想到,这很有可能是一句话,那么中间必然会有空格来分割单词
// 注意到空个也是高频字符之一,所以我们可以尝试在高频字符中找出那些位置可能是空格的地方
// 其中高频字符有0x36(3次), 0x37(5次), 0x78(6次)
// 0x36出现在倒数第二个字符, 0x37出现在倒数第一个字符,这些位置出现空格都是罕见或者不可能的
// 于是我尝试用0x78作为加密后的空格来尝试解密,当在最前面几个字符中得到Cooking时,显然这就对了
// 具体过程:
// 空格的ascii的二进制表示为0b0010_0000
// 将它与待解密的任意一个字节码做异或得到密钥0b0101_1000
// 0x15(0b0001_0101, 词频:1) ^ 0b0101_1000 -> 0b0100_1101 M
// 0x1b(0b0001_1011, 词频:2) ^ 0b0101_1000 -> 0b0100_0011 C
// 0x28(0b0010_1000, 词频:1) ^ 0b0101_1000 -> 0b0111_0000 p
// 0x2b(0b0010_1011, 词频:1) ^ 0b0101_1000 -> 0b0111_0011 s
// 0x2d(0b0010_1101, 词频:1) ^ 0b0101_1000 -> 0b0111_0101 u
// 0x31(0b0011_0001, 词频:2) ^ 0b0101_1000 -> 0b0110_1001 i
// 0x33(0b0011_0011, 词频:2) ^ 0b0101_1000 -> 0b0110_1011 k
// 0x34(0b0011_0100, 词频:1) ^ 0b0101_1000 -> 0b0110_1100 l
// 0x36(0b0011_0110, 词频:3) ^ 0b0101_1000 -> 0b0110_1110 n
// 0x37(0b0011_0111, 词频:5) ^ 0b0101_1000 -> 0b0110_1111 o
// 0x39(0b0011_1001, 词频:2) ^ 0b0101_1000 -> 0b0110_0001 a
// 0x3a(0b0011_1010, 词频:1) ^ 0b0101_1000 -> 0b0110_0010 b
// 0x3b(0b0011_1011, 词频:1) ^ 0b0101_1000 -> 0b0110_0011 c
// 0x3c(0b0011_1100, 词频:1) ^ 0b0101_1000 -> 0b0110_0100 d
// 0x3d(0b0011_1101, 词频:1) ^ 0b0101_1000 -> 0b0110_0101 e
// 0x3e(0b0011_1110, 词频:1) ^ 0b0101_1000 -> 0b0110_0110 f
// 0x3f(0b0011_1111, 词频:1) ^ 0b0101_1000 -> 0b0110_0111 g
// 0x78(0b0111_1000, 词频:6) ^ 0b0101_1000 -> 0b0010_0000 
// 0x7f(0b0111_1111, 词频:1) ^ 0b0101_1000 -> 0b0010_0111 '
//   1b 37 37 33 31 36 3f 78 15 1b 7f 2b 78 34 31 33 3d 78 39 78 28 37 2d 36 3c 78 37 3e 78 3a 39 3b 37 36
// ^ 58 ...
// --------------------------------------------------------------------------------------------------------
//    C  o  o  k  i  n  g     M  C  '  s     l  i  k  e     a     p  o  u  n  d     o  f     b  a  c  o  n
// BTW: Cooking MC's like a pound of bacon是Vanilla Ice在80年代的一首说唱歌曲Ice Ice Baby里的歌词


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