//! <https://www.codewars.com/kata/5899642f6e1b25935d000161/train/rust>

use core::hint::unreachable_unchecked;
use unchecked_std::prelude::*;

unsafe fn push_unchecked_if_not_last_unchecked(res: &mut Vec<i32>, x: i32) {
    if *res.get_unchecked(res.len() - 1) != x {
        res.push_unchecked(x);
    }
}

pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(arr1.len() + arr2.len());
    if arr1.is_empty() && arr2.is_empty() {
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
    unsafe {
        if arr1.is_empty() {
            if arr2.is_empty() {
                unreachable_unchecked();
            }
            if desc2 {
                res.push_unchecked(arr2[arr2.len() - 1]);
                for &e in arr2[..arr2.len() - 1].iter().rev() {
                    push_unchecked_if_not_last_unchecked(&mut res, e);
                }
            } else {
                res.push_unchecked(arr2[0]);
                for &e in &arr2[1..] {
                    push_unchecked_if_not_last_unchecked(&mut res, e);
                }
            }
            return res;
        }
        if arr2.is_empty() {
            if arr1.is_empty() {
                unreachable_unchecked();
            }
            if desc1 {
                res.push_unchecked(arr1[arr1.len() - 1]);
                for &e in arr1[..arr1.len() - 1].iter().rev() {
                    push_unchecked_if_not_last_unchecked(&mut res, e);
                }
            } else {
                res.push_unchecked(arr1[0]);
                for &e in &arr1[1..] {
                    push_unchecked_if_not_last_unchecked(&mut res, e);
                }
            }
            return res;
        }
        let mut e1 = *arr1.get_unchecked(i1);
        let mut e2 = *arr2.get_unchecked(i2);
        match (desc1, desc2) {
            (false, false) => {
                if e1 <= e2 {
                    res.push_unchecked(e1);
                    if i1 == arr1.len() - 1 {
                        for &e in arr2.get_unchecked(i2..) {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i1 += 1;
                    e1 = *arr1.get_unchecked(i1);
                } else {
                    res.push_unchecked(e2);
                    if i2 == arr2.len() - 1 {
                        for &e in arr1.get_unchecked(i1..) {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i2 += 1;
                    e2 = *arr2.get_unchecked(i2);
                }
                loop {
                    if e1 <= e2 {
                        push_unchecked_if_not_last_unchecked(&mut res, e1);
                        if i1 == arr1.len() - 1 {
                            for &e in arr2.get_unchecked(i2..) {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i1 += 1;
                        e1 = *arr1.get_unchecked(i1);
                    } else {
                        push_unchecked_if_not_last_unchecked(&mut res, e2);
                        if i2 == arr2.len() - 1 {
                            for &e in arr1.get_unchecked(i1..) {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i2 += 1;
                        e2 = *arr2.get_unchecked(i2);
                    }
                }
            }
            (false, true) => {
                if e1 <= e2 {
                    res.push_unchecked(e1);
                    if i1 == arr1.len() - 1 {
                        for &e in arr2.get_unchecked(..=i2).iter().rev() {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i1 += 1;
                    e1 = *arr1.get_unchecked(i1);
                } else {
                    res.push_unchecked(e2);
                    if i2 == 0 {
                        for &e in arr1.get_unchecked(i1..) {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i2 -= 1;
                    e2 = *arr2.get_unchecked(i2);
                }
                loop {
                    if e1 <= e2 {
                        push_unchecked_if_not_last_unchecked(&mut res, e1);
                        if i1 == arr1.len() - 1 {
                            for &e in arr2.get_unchecked(..=i2).iter().rev() {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i1 += 1;
                        e1 = *arr1.get_unchecked(i1);
                    } else {
                        push_unchecked_if_not_last_unchecked(&mut res, e2);
                        if i2 == 0 {
                            for &e in arr1.get_unchecked(i1..) {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i2 -= 1;
                        e2 = *arr2.get_unchecked(i2);
                    }
                }
            }
            (true, false) => {
                if e1 <= e2 {
                    res.push_unchecked(e1);
                    if i1 == 0 {
                        for &e in arr2.get_unchecked(i2..) {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i1 -= 1;
                    e1 = *arr1.get_unchecked(i1);
                } else {
                    res.push_unchecked(e2);
                    if i2 == arr2.len() - 1 {
                        for &e in arr1.get_unchecked(..=i1).iter().rev() {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i2 += 1;
                    e2 = *arr2.get_unchecked(i2);
                }
                loop {
                    if e1 <= e2 {
                        push_unchecked_if_not_last_unchecked(&mut res, e1);
                        if i1 == 0 {
                            for &e in arr2.get_unchecked(i2..) {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i1 -= 1;
                        e1 = *arr1.get_unchecked(i1);
                    } else {
                        push_unchecked_if_not_last_unchecked(&mut res, e2);
                        if i2 == arr2.len() - 1 {
                            for &e in arr1.get_unchecked(..=i1).iter().rev() {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i2 += 1;
                        e2 = *arr2.get_unchecked(i2);
                    }
                }
            }
            (true, true) => {
                if e1 <= e2 {
                    res.push_unchecked(e1);
                    if i1 == 0 {
                        for &e in arr2.get_unchecked(..=i2).iter().rev() {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i1 -= 1;
                    e1 = *arr1.get_unchecked(i1);
                } else {
                    res.push_unchecked(e2);
                    if i2 == 0 {
                        for &e in arr1.get_unchecked(..=i1).iter().rev() {
                            push_unchecked_if_not_last_unchecked(&mut res, e);
                        }
                        return res;
                    }
                    i2 -= 1;
                    e2 = *arr2.get_unchecked(i2);
                }
                loop {
                    if e1 <= e2 {
                        push_unchecked_if_not_last_unchecked(&mut res, e1);
                        if i1 == 0 {
                            for &e in arr2.get_unchecked(..=i2).iter().rev() {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i1 -= 1;
                        e1 = *arr1.get_unchecked(i1);
                    } else {
                        push_unchecked_if_not_last_unchecked(&mut res, e2);
                        if i2 == 0 {
                            for &e in arr1.get_unchecked(..=i1).iter().rev() {
                                push_unchecked_if_not_last_unchecked(&mut res, e);
                            }
                            return res;
                        }
                        i2 -= 1;
                        e2 = *arr2.get_unchecked(i2);
                    }
                }
            }
        }
    }
}
