//! <https://www.codewars.com/kata/5c1ae703ba76f438530000a2/train/rust>

pub fn word_mesh(words: &[&str]) -> Option<String> {
    // arbitrary capacity
    let cap = 8 * words.len();
    let mut res = String::with_capacity(cap);

    'pairs: for pair in words.windows(2) {
        let left = pair[0];
        let right = pair[1];

        for i in left.len().saturating_sub(right.len())..left.len() {
            if let Some(suffix) = left.get(i..) {
                if right.starts_with(suffix) {
                    res.push_str(suffix);
                    continue 'pairs;
                }
            }
        }

        return None;
    }

    Some(res)
}
