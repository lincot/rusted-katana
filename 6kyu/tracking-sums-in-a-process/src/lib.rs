//! <https://www.codewars.com/kata/56dbb6603e5dd6543c00098d/train/rust>

use unchecked_core::PushUnchecked;
use vqsort::VqSort;

pub fn track_sum(arr: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut tracked_sums = Vec::with_capacity(4);
    unsafe { tracked_sums.set_len(4) };

    if arr.is_empty() {
        for x in &mut tracked_sums {
            *x = 0;
        }
        return (tracked_sums, vec![]);
    }

    tracked_sums[0] = arr.iter().sum();
    let mut arr = arr.to_vec();
    VqSort::sort_descending(&mut arr);

    let mut dedup_arr = Vec::with_capacity(arr.len());
    let mut prev = arr[0];
    unsafe { dedup_arr.push_unchecked(prev) };
    tracked_sums[1] = prev;
    for &x in &arr[1..] {
        if x == prev {
            continue;
        }
        unsafe { dedup_arr.push_unchecked(x) };
        tracked_sums[1] += x;
        prev = x;
    }

    let mut diff_array = Vec::with_capacity(dedup_arr.len() - 1);
    unsafe { diff_array.set_len(dedup_arr.len() - 1) };
    tracked_sums[2] = 0;
    for i in 0..dedup_arr.len() - 1 {
        diff_array[i] = (i, dedup_arr[i] - dedup_arr[i + 1]);
        tracked_sums[2] += diff_array[i].1;
    }

    tracked_sums[3] = 0;
    if diff_array.len() < 64 {
        diff_array.sort_by_key(|&(_, x)| x);
    } else {
        radsort::sort_by_key(&mut diff_array, |&(_, x)| x);
    }
    let mut dedup_diff_array = Vec::with_capacity(diff_array.len());
    let mut diff_array = diff_array.into_iter();
    if let Some((i, x)) = diff_array.next() {
        unsafe { dedup_diff_array.push_unchecked((i, x)) };
        let mut prev = x;
        tracked_sums[3] += prev;
        for (i, x) in diff_array {
            if x == prev {
                continue;
            }
            unsafe { dedup_diff_array.push_unchecked((i, x)) };
            tracked_sums[3] += x;
            prev = x;
        }
    }
    if dedup_diff_array.len() < 64 {
        dedup_diff_array.sort_unstable_by_key(|&(i, _)| i);
    } else {
        radsort::sort_by_key(&mut dedup_diff_array, |&(i, _)| i);
    }

    (
        tracked_sums,
        dedup_diff_array.into_iter().map(|(_, x)| x).collect(),
    )
}
