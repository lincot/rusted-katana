//! <https://www.codewars.com/kata/5a61a846cadebf9738000076/train/rust>

pub fn peak(arr: &[u32]) -> Option<usize> {
    let mut sums = Vec::with_capacity(arr.len());
    let mut sum = 0;

    for n in arr {
        sums.push(sum);
        sum += n;
    }

    for (i, (n, sum_left)) in arr.iter().zip(sums).enumerate() {
        if sum_left == sum - n - sum_left {
            return Some(i);
        }
    }

    None
}
