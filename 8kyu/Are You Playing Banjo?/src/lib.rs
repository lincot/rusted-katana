//! <https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust>

use std::str::Bytes;

fn are_you_playing_banjo_output(text: &[u8], name: &str, bytes: Bytes, c: u8) -> String {
    let mut res = Vec::with_capacity(name.len() + text.len());

    res.push(c);
    res.extend(bytes);
    res.extend(text);

    unsafe { String::from_utf8_unchecked(res) }
}

pub fn are_you_playing_banjo(name: &str) -> String {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(c) => are_you_playing_banjo_output(
            if c == b'R' || c == b'r' {
                b" plays banjo"
            } else {
                b" does not play banjo"
            },
            name,
            bytes,
            c,
        ),
        None => panic!(),
    }
}
