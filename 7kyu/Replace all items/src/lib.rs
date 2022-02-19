//! <https://www.codewars.com/kata/57ae18c6e298a7a6d5000c7a/train/rust>

pub fn replace_all<T: PartialEq + Copy>(xs: &[T], find: T, replace: T) -> Vec<T> {
    xs.iter()
        .map(|&x| if x == find { replace } else { x })
        .collect()
}
