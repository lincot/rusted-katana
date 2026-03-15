//! <https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust>

pub const fn xo(string: &str) -> bool {
    let mut balance = 0isize;
    let mut i = 0;
    while i < string.len() {
        match string.as_bytes()[i] {
            b'x' | b'X' => balance += 1,
            b'o' | b'O' => balance -= 1,
            _ => {}
        }
        i += 1;
    }
    balance == 0
}
