//! <https://www.codewars.com/kata/5b1b27c8f60e99a467000041/train/rust>

use unchecked_std::prelude::*;

pub fn anagram_difference(w1: &str, w2: &str) -> u32 {
    if w1.as_bytes().iter().all(|&b| b.is_ascii_alphabetic())
        && w2.as_bytes().iter().all(|&b| b.is_ascii_alphabetic())
    {
        return unsafe { anagram_difference_ascii(w1, w2) };
    }

    let mut w1_chars = Vec::with_capacity(w1.len());
    for ch in w1.chars() {
        unsafe { w1_chars.push_unchecked((ch, false)) };
    }
    let mut res = w1_chars.len() as u32;
    for w2_ch in w2.chars() {
        if let Some((_, used)) = w1_chars
            .iter_mut()
            .find(|&&mut (w1_ch, used)| !used && w1_ch == w2_ch)
        {
            *used = true;
            res -= 1;
        } else {
            res += 1;
        }
    }
    res
}

unsafe fn anagram_difference_ascii(w1: &str, w2: &str) -> u32 {
    let mut char_counts = [0i32; 26];
    for &ch in w1.as_bytes() {
        *char_counts.get_unchecked_mut((ch - b'a') as usize) += 1;
    }
    for &ch in w2.as_bytes() {
        *char_counts.get_unchecked_mut((ch - b'a') as usize) -= 1;
    }
    char_counts.into_iter().map(i32::unsigned_abs).sum()
}
