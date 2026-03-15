//! <https://www.codewars.com/kata/5ce399e0047a45001c853c2b/train/rust>

pub fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut res = Vec::with_capacity(ls.len() + 1);
    let spare = res.spare_capacity_mut();
    unsafe { spare.get_unchecked_mut(ls.len()) }.write(0);
    for i in (0..ls.len()).rev() {
        unsafe {
            let prev = spare.get_unchecked(i + 1).assume_init();
            spare.get_unchecked_mut(i).write(ls[i] + prev);
        }
    }
    unsafe { res.set_len(ls.len() + 1) };
    res
}
