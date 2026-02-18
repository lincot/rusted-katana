//! <https://www.codewars.com/kata/5aa3e2b0373c2e4b420009af/train/rust>

use unchecked_std::prelude::*;

pub fn tops(msg: &str) -> String {
    if msg.is_ascii() {
        return unsafe { String::from_utf8_unchecked(tops_bytes(msg.as_bytes())) };
    }

    let mut tops = Vec::with_capacity(get_tops_count(msg.as_bytes()));
    let mut till_top = 2usize;
    let mut next_till_top = 3;
    let mut top_start = 0;
    let mut block_width = -1isize;
    let mut next_block_width = 2;
    for (i, _) in msg.char_indices() {
        if till_top == 0 {
            next_till_top += 4;
            till_top = next_till_top;
            top_start = i;
            block_width = next_block_width;
            next_block_width += 1;
        }
        till_top -= 1;

        if block_width == 0 {
            unsafe { tops.push_unchecked((top_start, i)) };
        }
        block_width -= 1;
    }
    if block_width >= 0 {
        unsafe { tops.push_unchecked((top_start, msg.len())) };
    }

    let mut res = String::with_capacity(msg.len());
    let Some(&(top_start, top_end)) = tops.last() else {
        return res;
    };
    unsafe { res.push_str_unchecked(msg.get_unchecked(top_start..top_end.min(msg.len()))) };
    for &(top_start, top_end) in tops[..tops.len() - 1].iter().rev() {
        unsafe { res.push_str_unchecked(msg.get_unchecked(top_start..top_end)) };
    }
    res
}

fn tops_bytes(msg: &[u8]) -> Vec<u8> {
    let n_tops = get_tops_count(msg);
    if n_tops == 0 {
        return Vec::new();
    }

    let mut res = Vec::with_capacity(n_tops * (n_tops + 3) / 2);
    let i = n_tops - 1;
    let mut start = 2 * i * i + 5 * i + 2;
    // may not hold for very big strings
    if start < msg.len() {
        unsafe { res.extend_from_slice_unchecked(&msg[start..(start + i + 2).min(msg.len())]) };
    }
    for i in (0..i).rev() {
        start -= 4 * i + 7;
        unsafe { res.extend_from_slice_unchecked(msg.get_unchecked(start..start + i + 2)) };
    }
    res
}

/// Get the number of tops. May return 1 more than the actual number for really big strings.
fn get_tops_count(msg: &[u8]) -> usize {
    (unsafe {
        (msg.len() as f64)
            .mul_add(8., 1.)
            .sqrt()
            .to_int_unchecked::<usize>()
    } - 1)
        / 4
}
