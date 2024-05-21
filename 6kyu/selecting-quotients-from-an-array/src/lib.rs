//! <https://www.codewars.com/kata/569f6ad962ff1dd52f00000d/train/rust>

use vqsort::VqSort;

#[derive(PartialEq, Eq)]
enum Parity {
    Even,
    Odd,
    Any,
}

pub fn select_quotients(arr: &[u32], m: u32, dir_str: &str) -> Vec<(u32, (u32, u32))> {
    assert!(m != 0 && !arr.is_empty());

    let mut arr = arr.to_vec().into_boxed_slice();
    VqSort::sort(&mut arr);
    let mut res = Vec::new();

    let parity = if dir_str.is_empty() {
        Parity::Any
    } else if b"eE".contains(&dir_str.as_bytes()[0]) {
        Parity::Even
    } else {
        Parity::Odd
    };

    let mut prev_num = 0;
    for i in 0..arr.len() {
        let num = arr[i];
        if num == prev_num {
            continue;
        }
        let mut prev_denom = 0;

        let partition_point =
            unsafe { arr.get_unchecked(..i) }.partition_point(|&denom| num / m >= denom);
        for &denom in unsafe { arr.get_unchecked(..partition_point) } {
            if denom != prev_denom
                && denom != 0
                && num % denom == 0
                && (parity != Parity::Even || num / denom % 2 == 0)
                && (parity != Parity::Odd || num / denom % 2 == 1)
            {
                res.push((num / denom, (num, denom)));
            }

            prev_denom = denom;
        }
        prev_num = num;
    }

    if res.len() < 64 {
        res.sort_by_key(|&(q, _)| q);
    } else {
        radsort::sort_by_key(&mut res, |&(q, _)| q);
    }
    res
}
