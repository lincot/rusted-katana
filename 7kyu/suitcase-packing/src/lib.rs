//! <https://www.codewars.com/kata/5c556845d7e0334c74698706/train/rust>

pub fn fit_in(a: u32, b: u32, m: u32, n: u32) -> bool {
    a + b <= m.max(n) && a.max(b) <= m.min(n)
}
