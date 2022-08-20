//! <https://www.codewars.com/kata/55902c5eaa8069a5b4000083/train/rust>

use my_prelude::prelude::*;

pub fn format_money(amount: f64) -> String {
    let mut res = String::with_capacity("$184467440737095516.15".len());
    let amount = (100.000_000_1 * amount) as u64;
    unsafe {
        res.push_unchecked('$');
        res.write_num_unchecked(amount);
        for _ in 0..3usize.saturating_sub(res.len() - 1) {
            res.push_unchecked('0');
        }
        let len_before = res.len();
        let last = *res.as_bytes().get_unchecked(len_before - 1);
        let pen = *res.as_bytes().get_unchecked(len_before - 2);
        res.as_mut_vec().push_unchecked(last);
        *res.as_mut_vec().get_unchecked_mut(len_before - 1) = pen;
        *res.as_mut_vec().get_unchecked_mut(len_before - 2) = b'.';
    }
    res
}
