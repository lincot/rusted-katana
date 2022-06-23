//! <https://www.codewars.com/kata/57fd696e26b06857eb0011e7/train/rust>

pub fn dative(word: &str) -> String {
    let mut res = String::with_capacity(word.len() + 3);
    res.push_str(word);

    for b in word.bytes().rev() {
        match b {
            b'e' | 0xa9 | b'i' | 0xad | 0xb6 | 0x91 | 0xbc | 0xb1 => {
                res.push_str("nek");
                break;
            }
            b'a' | 0xa1 | b'o' | 0xb3 | b'u' | 0xba => {
                res.push_str("nak");
                break;
            }
            _ => {}
        }
    }

    res
}
