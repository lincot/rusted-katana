//! <https://www.codewars.com/kata/542ebbdb494db239f8000046/train/rust>

pub fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    for pair in slice.windows(2) {
        if pair[0] == find {
            return Some(pair[1].clone());
        }
    }
    None
}
