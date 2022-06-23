//! <https://www.codewars.com/kata/5a651865fd56cb55760000e0/train/rust>

pub fn array_leaders(arr: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(arr.len());
    let mut sum = 0;

    for &x in arr.iter().rev() {
        if x > sum {
            res.push(x);
        }

        sum += x;
    }

    res.reverse();

    res
}
