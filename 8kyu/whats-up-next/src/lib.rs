//! <https://www.codewars.com/kata/542ebbdb494db239f8000046/train/rust>

#![feature(array_windows)]

pub fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    for [a, b] in slice.array_windows() {
        if a == &find {
            return Some(b.clone());
        }
    }
    None
}
