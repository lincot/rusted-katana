//! <https://www.codewars.com/kata/5a2fd38b55519ed98f0000ce/train/rust>

use my_prelude::prelude::*;

pub fn multi_table(n: u64) -> String {
    let mut repeating = String::with_capacity(" *  = ".len() + 20);
    unsafe {
        repeating.push_str_unchecked(" * ");
        repeating.write_num_unchecked(n);
        repeating.push_str_unchecked(" = ");
    }

    // worst case capacity, may be 8 less
    let cap = 20 * repeating.len() - 31;
    let mut res = String::with_capacity(cap);
    unsafe {
        res.push_unchecked('1');
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(n);
        res.push_str_unchecked("\n2");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(2 * n);
        res.push_str_unchecked("\n3");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(3 * n);
        res.push_str_unchecked("\n4");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(4 * n);
        res.push_str_unchecked("\n5");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(5 * n);
        res.push_str_unchecked("\n6");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(6 * n);
        res.push_str_unchecked("\n7");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(7 * n);
        res.push_str_unchecked("\n8");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(8 * n);
        res.push_str_unchecked("\n9");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(9 * n);
        res.push_str_unchecked("\n10");
        res.push_str_unchecked(&repeating);
        res.write_num_unchecked(10 * n);
    }
    res
}
