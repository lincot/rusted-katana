//! <https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust>

pub fn get_middle(s: &str) -> &str {
    if s.is_empty() {
        return s;
    }

    if s.is_ascii() {
        return unsafe { core::str::from_utf8_unchecked(get_middle_bytes(s.as_bytes())) };
    }

    let count = s.chars().count();
    let mut char_indices = s.char_indices().skip((count - 1) / 2);

    unsafe {
        let (idx, ch) = char_indices.next().unwrap_unchecked();
        if count.is_multiple_of(2) {
            let (_, ch2) = char_indices.next().unwrap_unchecked();
            s.get_unchecked(idx..idx + ch.len_utf8() + ch2.len_utf8())
        } else {
            s.get_unchecked(idx..idx + ch.len_utf8())
        }
    }
}

unsafe fn get_middle_bytes(s: &[u8]) -> &[u8] {
    unsafe { s.get_unchecked((s.len() - 1) / 2..s.len() / 2 + 1) }
}
