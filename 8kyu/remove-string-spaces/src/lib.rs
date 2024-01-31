//! <https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/rust>

pub fn no_space(mut x: String) -> String {
    let vec = unsafe { x.as_mut_vec() };
    vec.retain(|&c| c != b' ');
    x
}
