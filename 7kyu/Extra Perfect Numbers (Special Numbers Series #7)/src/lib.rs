//! <https://www.codewars.com/kata/5a662a02e626c54e87000123/train/rust>

pub fn extra_perfect(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(((n + 1) / 2) as usize);

    let mut i = 1;
    while i <= n {
        res.push(i);
        i += 2;
    }

    res
}
