//! <https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked::PushStrUnchecked;

pub fn number(lines: &[&str]) -> Vec<String> {
    (1usize..)
        .zip(lines)
        .map(|(line_number, line)| unsafe {
            let mut numbered_line =
                String::with_capacity(usize::MAX_LEN_BASE10 + ": ".len() + line.len());
            numbered_line.write_num_unchecked(line_number, 10, false, false);
            numbered_line.push_str_unchecked(": ");
            numbered_line.push_str_unchecked(line);
            numbered_line
        })
        .collect()
}
