//! <https://www.codewars.com/kata/556deca17c58da83c00002db/train/rust>

pub fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut res = Vec::with_capacity(n);
    res.extend(signature.iter().take(n));

    for i in 3..n {
        if i != res.len() || res.len() < 3 {
            unsafe { core::hint::unreachable_unchecked() };
        }
        res.push(res[i - 3..].iter().sum());
    }

    res
}
