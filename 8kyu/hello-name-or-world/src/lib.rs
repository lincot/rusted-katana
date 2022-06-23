//! <https://www.codewars.com/kata/57e3f79c9cb119374600046b/train/rust>

const HELLO: &str = "Hello, ";

pub fn hello(name: &str) -> String {
    let mut name_chars = name.chars();

    let first = match name_chars.next() {
        Some(c) => c,
        None => return "Hello, World!".into(),
    };

    // will be this length most probably
    let cap = HELLO.len() + name.len() + 1;
    let mut res = String::with_capacity(cap);

    res.push_str(HELLO);

    if first.is_lowercase() {
        res.extend(first.to_uppercase());
    } else {
        res.push(first);
    }

    for c in name_chars {
        if c.is_uppercase() {
            res.extend(c.to_lowercase());
        } else {
            res.push(c);
        }
    }

    res.push('!');

    res
}
