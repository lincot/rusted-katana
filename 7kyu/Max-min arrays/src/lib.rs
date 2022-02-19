//! <https://www.codewars.com/kata/5a090c4e697598d0b9000004/train/rust>

pub fn solve(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    arr.sort_unstable();

    let mut res = Vec::with_capacity(arr.len());
    let mut it = arr.into_iter();

    while let Some(x) = it.next_back() {
        res.push(x);
        if let Some(x) = it.next() {
            res.push(x);
        }
    }

    res
}
