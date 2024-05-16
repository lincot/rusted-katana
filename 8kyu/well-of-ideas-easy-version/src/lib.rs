//! <https://www.codewars.com/kata/57f222ce69e09c3630000212/train/rust>

pub fn well(x: &[&str]) -> &'static str {
    match x.iter().filter(|idea| idea.starts_with('g')).count() {
        0 => "Fail!",
        1..=2 => "Publish!",
        _ => "I smell a series!",
    }
}
