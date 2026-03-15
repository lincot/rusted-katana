//! <https://www.codewars.com/kata/5d50e3914861a500121e1958/train/rust>

pub fn add_letters(letters: Vec<char>) -> char {
    const A: u32 = 'a' as u32 - 1;
    const Z: char = 'z';
    let sum = letters.iter().map(|&c| c as u32 - A).sum::<u32>() % (Z as u32 - A);
    if sum == 0 {
        return Z;
    }
    char::from_u32(sum + A).unwrap()
}
