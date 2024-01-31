//! <https://www.codewars.com/kata/64f41ad92b610b64c1067590/train/rust>

pub fn full_cycle(lst: &[usize]) -> bool {
    if lst.is_empty() {
        return true;
    }
    let mut i = 0;
    for _ in 0..lst.len() - 1 {
        i = lst[i];
        if i == 0 {
            return false;
        }
    }
    true
}
