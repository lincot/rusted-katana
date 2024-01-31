//! <https://www.codewars.com/kata/570b69d96731d4cf9c001597/train/rust>

pub fn magic_square(n: u32) -> Vec<Vec<u32>> {
    let n = n as usize;
    if n == 0 {
        return Vec::new();
    }
    let mut res = vec![vec![0; n]; n];
    let (mut i, mut j) = (0, n / 2);
    for x in 0..n as u32 * n as u32 {
        unsafe { *res.get_unchecked_mut(i).get_unchecked_mut(j) = x + 1 };
        let new_i = if i == 0 { n - 1 } else { i - 1 };
        let new_j = if j + 1 == n { 0 } else { j + 1 };
        if unsafe { *res.get_unchecked(new_i).get_unchecked(new_j) } == 0 {
            (i, j) = (new_i, new_j);
        } else {
            i = if i + 1 == n { 0 } else { i + 1 };
        }
    }
    res
}
