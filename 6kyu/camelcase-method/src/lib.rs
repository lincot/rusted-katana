//! <https://www.codewars.com/kata/587731fda577b3d1b0001196/train/rust>

pub fn camel_case(str: &str) -> String {
    let mut res = String::with_capacity(str.len());

    let mut str = str.chars();

    for c in str.by_ref() {
        if c != ' ' {
            res.extend(c.to_uppercase());
            break;
        }
    }
    while let Some(c) = str.next() {
        if c == ' ' {
            for c in str.by_ref() {
                if c != ' ' {
                    res.extend(c.to_uppercase());
                    break;
                }
            }
        } else {
            res.push(c);
        }
    }

    res
}
