use std::cmp; 

pub fn challenge1(hex: &str) -> String {
    let table = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];
    // 
    let padding = ["", "", "==", "", "=", ""];

    let mut hold = 0u8; 
    let mut offset = 0; 
    let mut code: u8;
    
    let len = hex.len();
    // hex转换为bytes之后的位长度
    let bit_len = if len % 2 == 1 { len + 1 } else { len } * 4;
    let mut result = String::with_capacity((bit_len + 5) / 6 + padding[bit_len % 6].len());

    let mut from = 0;
    let mut to = cmp::min(len, 2);
    while to <= len {
        let parsed = u8::from_str_radix(&hex[from..to], 16).unwrap();

        code = ((parsed << 0) | hold) & 0b0011_1111;
        hold = parsed >> (6 - offset);
        offset += 2;
        result.push(table[code as usize]);

        if offset == 6 {
            result.push(table[hold as usize]);
            hold = 0;
            offset = 0;
        }
        

        from = to;
        to = cmp::min(len, to + 2);
    }

    result.push_str(padding[bit_len % 6]);
    
    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_challenge1() {
        let base64_value = challenge1("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(base64_value, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}