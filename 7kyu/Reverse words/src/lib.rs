//! <https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust>

pub fn reverse_words(str: &str) -> String {
    let mut res = String::with_capacity(str.len());

    let mut reversed_words = str.as_bytes().split(|&b| b == b' ').map(|bytes| {
        unsafe { std::str::from_utf8_unchecked(bytes) }
            .chars()
            .rev()
    });

    if let Some(w) = reversed_words.next() {
        res.extend(w);
    }

    reversed_words.for_each(|w| {
        res.push(' ');
        res.extend(w);
    });

    res
}
