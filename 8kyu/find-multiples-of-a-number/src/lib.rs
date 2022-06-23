//! <https://www.codewars.com/kata/58ca658cc0d6401f2700045f/train/rust>

pub fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let cap = (limit / n) as usize;
    let mut res = Vec::with_capacity(cap);

    let mut i = n;
    while i <= limit {
        res.push(i);
        i += n;
    }

    res
}
