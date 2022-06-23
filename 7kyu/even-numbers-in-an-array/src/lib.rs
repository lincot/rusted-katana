//! <https://www.codewars.com/kata/5a431c0de1ce0ec33a00000c/train/rust>

pub fn even_numbers(array: &[i32], number: usize) -> Vec<i32> {
    let mut res: Vec<_> = array
        .iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .rev()
        .take(number)
        .collect();
    res.reverse();
    res
}
