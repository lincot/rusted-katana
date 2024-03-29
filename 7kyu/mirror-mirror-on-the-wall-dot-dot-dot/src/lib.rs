//! <https://www.codewars.com/kata/5f55ecd770692e001484af7d/train/rust>

use unchecked_core::{ExtendFromSliceUnchecked, PushUnchecked};
use vqsort::VqSort;

pub fn mirror(list: &[i32]) -> Vec<i32> {
    if list.len() <= 1 {
        return list.into();
    }

    let mut res = Vec::with_capacity(2 * list.len() - 1);

    unsafe { res.extend_from_slice_unchecked(list) };
    VqSort::sort_ascending(&mut res);

    for i in (0..res.len() - 1).rev() {
        unsafe {
            res.push_unchecked(*res.get_unchecked(i));
        }
    }

    res
}
