//! <https://www.codewars.com/kata/5b049d57de4c7f6a6c0001d7/train/rust>

pub fn apparently(string: &str) -> String {
    // arbitrary capacity
    let cap = string.len() + string.len() / 3;
    let mut res = String::with_capacity(cap);

    let words = string.split_ascii_whitespace().collect::<Vec<_>>();

    if let Some(x) = words.get(0) {
        res.push_str(x);
    } else {
        return res;
    }

    for i in 1..words.len() {
        if ["and", "but"].contains(unsafe { words.get_unchecked(i - 1) })
            && *unsafe { words.get_unchecked(i) } != "apparently"
        {
            res.push_str(" apparently");
        }

        res.push(' ');
        res.push_str(unsafe { words.get_unchecked(i) });
    }

    if ["and", "but"].contains(unsafe { words.get_unchecked(words.len() - 1) }) {
        res.push_str(" apparently");
    }

    res
}
