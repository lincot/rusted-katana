//! <https://www.codewars.com/kata/590c4c342ad5cd6a8700005c/train/rust>

pub fn prefix_sums_to_suffix_sums(prefix_sums: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(prefix_sums.len());
    let Some(last) = prefix_sums.last().copied() else {
        return res;
    };
    let spare = res.spare_capacity_mut();
    unsafe { spare.get_unchecked_mut(0) }.write(last);
    let mut i = 0;
    while i < prefix_sums.len() - 1 {
        unsafe { spare.get_unchecked_mut(i + 1) }.write(last - prefix_sums[i]);
        i += 1;
    }
    unsafe { res.set_len(prefix_sums.len()) };
    res
}
