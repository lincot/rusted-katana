//! <https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust>

fn push_uppercase(s: &mut String, c: char) {
    if c.is_lowercase() {
        s.extend(c.to_uppercase());
    } else {
        s.push(c);
    }
}

pub fn abbrev_name(name: &str) -> String {
    let first = name.chars().next().unwrap();
    let first_len = first.len_utf8();

    let space_pos = name[first_len..].bytes().position(|b| b == b' ').unwrap() + first_len;

    let last = name[space_pos + 1..].chars().next();
    let last = unsafe { last.unwrap_unchecked() };

    // worst case capacity
    let cap = 6 + 1 + 6;
    let mut res = String::with_capacity(cap);

    push_uppercase(&mut res, first);
    res.push('.');
    push_uppercase(&mut res, last);

    res
}
