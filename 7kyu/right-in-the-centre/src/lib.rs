//! <https://www.codewars.com/kata/5f5da7a415fbdc0001ae3c69/train/rust>

pub fn is_in_middle(seq: &str) -> bool {
    if seq.len() < 3 {
        return false;
    }

    let i = (seq.len() - 3) / 2;

    (unsafe { seq.get_unchecked(i..i + 3) } == "abc")
        || (seq.len() % 2 == 0 && unsafe { seq.get_unchecked(i + 1..i + 4) } == "abc")
}
