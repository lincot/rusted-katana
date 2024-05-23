//! <https://www.codewars.com/kata/592a6ad46d6c5a62b600003f/train/rust>

pub fn encode(text: &str) -> String {
    let mut res = Vec::with_capacity(text.len());
    unsafe { res.set_len(text.len()) };
    let mut i = 0;
    while i < text.len() {
        res[i] = encode_byte(text.as_bytes()[i]);
        i += 1;
    }
    unsafe { String::from_utf8_unchecked(res) }
}

pub fn decode(text: &str) -> String {
    encode(text)
}

const fn encode_byte(b: u8) -> u8 {
    match b {
        b'G' => b'A',
        b'g' => b'a',
        b'a' => b'g',
        b'A' => b'G',
        b'D' => b'E',
        b'd' => b'e',
        b'e' => b'd',
        b'E' => b'D',
        b'R' => b'Y',
        b'r' => b'y',
        b'y' => b'r',
        b'Y' => b'R',
        b'P' => b'O',
        b'p' => b'o',
        b'o' => b'p',
        b'O' => b'P',
        b'L' => b'U',
        b'l' => b'u',
        b'u' => b'l',
        b'U' => b'L',
        b'K' => b'I',
        b'k' => b'i',
        b'i' => b'k',
        b'I' => b'K',
        b => b,
    }
}
