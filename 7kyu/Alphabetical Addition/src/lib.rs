//! <https://www.codewars.com/kata/5d50e3914861a500121e1958/train/rust>

pub fn add_letters(letters: Vec<char>) -> char {
    const A: usize = 'a' as usize - 1;
    const Z: char = 'z';

    match letters.iter().map(|&c| c as usize - A).sum::<usize>() % (Z as usize - A) {
        0 => Z,
        i => char::from_u32((i + A) as _).unwrap(),
    }
}
