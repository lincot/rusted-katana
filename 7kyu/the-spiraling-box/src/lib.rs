//! <https://www.codewars.com/kata/63b84f54693cb10065687ae5/train/rust>

pub fn create_box(m: u32, n: u32) -> Vec<Vec<u32>> {
    (0..n)
        .map(|i| {
            (0..m)
                .map(|j| (i + 1).min(n - i).min(j + 1).min(m - j))
                .collect()
        })
        .collect()
}
