//! <https://www.codewars.com/kata/5a2fd38b55519ed98f0000ce/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_core::{ExtendFromSliceUnchecked, PushStrUnchecked, PushUnchecked};

pub fn multi_table(n: u64) -> String {
    let mut repeating = heapless::Vec::<_, { " *  = ".len() + u64::MAX_LEN_BASE10 }>::new();
    let repeating = unsafe {
        repeating.extend_from_slice_unchecked(b" * ");
        repeating.write_num_unchecked(n, 10, false, false);
        repeating.extend_from_slice_unchecked(b" = ");
        core::str::from_utf8_unchecked(&repeating)
    };

    let cap = 20 * repeating.len() - 31;
    let mut res = String::with_capacity(cap);
    unsafe {
        res.push_unchecked('1');
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(n, 10, false, false);
        res.push_str_unchecked("\n2");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(2 * n, 10, false, false);
        res.push_str_unchecked("\n3");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(3 * n, 10, false, false);
        res.push_str_unchecked("\n4");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(4 * n, 10, false, false);
        res.push_str_unchecked("\n5");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(5 * n, 10, false, false);
        res.push_str_unchecked("\n6");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(6 * n, 10, false, false);
        res.push_str_unchecked("\n7");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(7 * n, 10, false, false);
        res.push_str_unchecked("\n8");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(8 * n, 10, false, false);
        res.push_str_unchecked("\n9");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(9 * n, 10, false, false);
        res.push_str_unchecked("\n10");
        res.push_str_unchecked(repeating);
        res.write_num_unchecked(10 * n, 10, false, false);
    }
    res
}
