//! <https://www.codewars.com/kata/542ebbdb494db239f8000046/train/rust>

pub fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    let mut slice = slice.iter();

    while slice.next() != Some(&find) {}

    slice.next().cloned()
}
