//! <https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust>

pub fn xo(string: &'static str) -> bool {
    let mut balance = 0isize;

    for b in string.bytes() {
        match b {
            b'x' | b'X' => balance += 1,
            b'o' | b'O' => balance -= 1,
            _ => {}
        }
    }

    balance == 0
}
