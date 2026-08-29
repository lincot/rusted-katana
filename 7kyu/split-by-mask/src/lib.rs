//! <https://www.codewars.com/kata/6a68ed7894f29599a1f7a248/train/rust>

use unchecked_std::prelude::*;

pub fn split(xs: &str, mask: &[usize]) -> Option<Vec<String>> {
    let mut pos = 0;
    let splits = mask
        .iter()
        .try_fold(Vec::with_capacity(mask.len()), |mut acc, len| {
            let end = len.checked_add(pos)?;
            if !xs.is_char_boundary(end) {
                return None;
            }
            unsafe { acc.push_unchecked(xs.get_unchecked(pos..end).into()) };
            pos = end;
            Some(acc)
        });
    if pos == xs.len() { splits } else { None }
}
