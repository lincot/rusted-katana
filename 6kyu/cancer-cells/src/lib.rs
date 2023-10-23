//! <https://www.codewars.com/kata/5931614bb2f657c18c0001c3/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use unchecked::PushUnchecked;

pub fn cut_cancer_cells(organism: &str) -> String {
    let mut res = String::with_capacity(organism.len());
    for (i, c) in organism.char_indices() {
        if !['c', 'C'].contains(&c)
            && (c.is_uppercase()
                || (i == 0 || *unsafe { organism.as_bytes().get_unchecked(i - 1) } != b'C')
                    && organism.as_bytes().get(i + c.len_utf8()) != Some(&b'C'))
        {
            unsafe { res.push_unchecked(c) };
        }
    }
    res
}
