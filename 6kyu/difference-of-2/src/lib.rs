//! <https://www.codewars.com/kata/5340298112fa30e786000688/train/rust>

use unchecked_std::PushUnchecked;

pub fn twos_difference(arr: &[u32]) -> Vec<(u32, u32)> {
    let mut arr = arr.to_vec();
    vqsort_rs::sort(&mut arr);
    let mut res = Vec::with_capacity(arr.len());
    for &[a, b, c] in arr.array_windows() {
        if a + 2 == b {
            unsafe { res.push_unchecked((a, b)) };
        } else if a + 2 == c {
            unsafe { res.push_unchecked((a, c)) };
        }
    }
    if arr.len() >= 2 {
        let (a, b) = (arr[arr.len() - 2], arr[arr.len() - 1]);
        if a + 2 == b {
            unsafe { res.push_unchecked((a, b)) };
        }
    }
    res
}
