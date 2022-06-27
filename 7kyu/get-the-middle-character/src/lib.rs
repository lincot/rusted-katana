//! <https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust>

pub fn get_middle(s: &str) -> &str {
    let char_indices: Vec<_> = s.char_indices().map(|(i, _)| i).collect();
    let len = char_indices.len();

    if len < 3 {
        s
    } else {
        unsafe {
            s.get_unchecked(
                *char_indices.get_unchecked((len - 1) / 2)
                    ..*char_indices.get_unchecked(len / 2 + 1),
            )
        }
    }
}
