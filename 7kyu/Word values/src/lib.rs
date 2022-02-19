//! <https://www.codewars.com/kata/598d91785d4ce3ec4f000018/train/rust>

pub fn word_value(words: &[&str]) -> Vec<i32> {
    (1..)
        .zip(words)
        .map(|(i, word)| {
            i * word
                .bytes()
                .map(|b| {
                    if (b'a'..=b'z').contains(&b) {
                        (b - b'a' + 1) as _
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .collect()
}
