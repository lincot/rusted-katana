//! <https://www.codewars.com/kata/523f5d21c841566fde000009/train/rust>

pub fn array_diff<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.retain(|x| !b.contains(x));
    a
}
