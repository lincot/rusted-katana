//! <https://www.codewars.com/kata/5899642f6e1b25935d000161/train/rust>

use my_prelude::prelude::*;

unsafe fn push_unchecked_if_new(res: &mut Vec<i32>, x: i32) {
    if res.last() != Some(&x) {
        res.push_unchecked(x);
    }
}

unsafe fn extend_unchecked_if_new(res: &mut Vec<i32>, arr: &[i32], i: usize, desc: bool) {
    if desc {
        for &e in arr.get_unchecked(..=i).iter().rev() {
            push_unchecked_if_new(res, e);
        }
    } else {
        for &e in arr.get_unchecked(i..) {
            push_unchecked_if_new(res, e);
        }
    }
}

pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(arr1.len() + arr2.len());

    if arr1.is_empty() {
        unsafe { res.extend_from_slice_unchecked(arr2) };
        return res;
    }
    if arr2.is_empty() {
        unsafe { res.extend_from_slice_unchecked(arr1) };
        return res;
    }

    let (mut i1, mut desc1) = (0, false);
    for i in 1..arr1.len() {
        if arr1[i - 1] != arr1[i] {
            if arr1[i - 1] > arr1[i] {
                (i1, desc1) = (arr1.len() - 1, true);
            }
            break;
        }
    }

    let (mut i2, mut desc2) = (0, false);
    for i in 1..arr2.len() {
        if arr2[i - 1] != arr2[i] {
            if arr2[i - 1] > arr2[i] {
                (i2, desc2) = (arr2.len() - 1, true);
            }
            break;
        }
    }

    loop {
        let e1 = *unsafe { arr1.get_unchecked(i1) };
        let e2 = *unsafe { arr2.get_unchecked(i2) };

        if e1 <= e2 {
            unsafe { push_unchecked_if_new(&mut res, e1) };
            if desc1 {
                if i1 == 0 {
                    unsafe { extend_unchecked_if_new(&mut res, arr2, i2, desc2) };
                    return res;
                }
                i1 -= 1;
            } else {
                if i1 == arr1.len() - 1 {
                    unsafe { extend_unchecked_if_new(&mut res, arr2, i2, desc2) };
                    return res;
                }
                i1 += 1;
            }
        } else {
            unsafe { push_unchecked_if_new(&mut res, e2) };
            if desc2 {
                if i2 == 0 {
                    unsafe { extend_unchecked_if_new(&mut res, arr1, i1, desc1) };
                    return res;
                }
                i2 -= 1;
            } else {
                if i2 == arr2.len() - 1 {
                    unsafe { extend_unchecked_if_new(&mut res, arr1, i1, desc1) };
                    return res;
                }
                i2 += 1;
            }
        }
    }
}
