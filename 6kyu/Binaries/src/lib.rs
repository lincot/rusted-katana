//! <https://www.codewars.com/kata/5d98b6b38b0f6c001a461198/train/rust>

pub fn code(s: &str) -> String {
    let mut res = Vec::with_capacity(8 * s.len());

    for b in s.bytes() {
        match b {
            b'0' => res.extend(b"10"),
            b'1' => res.extend(b"11"),
            b'2' => res.extend(b"0110"),
            b'3' => res.extend(b"0111"),
            b'4' => res.extend(b"001100"),
            b'5' => res.extend(b"001101"),
            b'6' => res.extend(b"001110"),
            b'7' => res.extend(b"001111"),
            b'8' => res.extend(b"00011000"),
            _ => res.extend(b"00011001"),
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}

pub fn decode(s: &str) -> String {
    let s = s.as_bytes();
    let mut res = Vec::with_capacity(s.len() / 2);

    let mut i = 0;
    'digits: while i < s.len() {
        if s[i..].starts_with(b"10") {
            res.push(b'0');
            i += 2;
            continue 'digits;
        }
        if s[i..].starts_with(b"11") {
            res.push(b'1');
            i += 2;
            continue 'digits;
        }
        if s[i..].starts_with(b"0110") {
            res.push(b'2');
            i += 4;
            continue 'digits;
        }
        if s[i..].starts_with(b"0111") {
            res.push(b'3');
            i += 4;
            continue 'digits;
        }
        if s[i..].starts_with(b"001100") {
            res.push(b'4');
            i += 6;
            continue 'digits;
        }
        if s[i..].starts_with(b"001101") {
            res.push(b'5');
            i += 6;
            continue 'digits;
        }
        if s[i..].starts_with(b"001110") {
            res.push(b'6');
            i += 6;
            continue 'digits;
        }
        if s[i..].starts_with(b"001111") {
            res.push(b'7');
            i += 6;
            continue 'digits;
        }
        if s[i..].starts_with(b"00011000") {
            res.push(b'8');
            i += 8;
            continue 'digits;
        }
        if s[i..].starts_with(b"00011001") {
            res.push(b'9');
            i += 8;
            continue 'digits;
        }
        panic!();
    }

    unsafe { String::from_utf8_unchecked(res) }
}
