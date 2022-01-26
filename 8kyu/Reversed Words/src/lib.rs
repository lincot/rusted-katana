//! <https://www.codewars.com/kata/51c8991dee245d7ddf00000e/train/rust>

pub fn reverse_words(words: &str) -> String {
    let cap = words.len();
    let mut res = Vec::with_capacity(cap);

    let mut words = words.as_bytes().split(|&c| c == b' ').rev();

    if let Some(word) = words.next() {
        res.extend(word);
    }

    words.for_each(|word| {
        res.push(b' ');
        res.extend(word);
    });

    unsafe { String::from_utf8_unchecked(res) }
}
