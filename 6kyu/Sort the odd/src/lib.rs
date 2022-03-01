//! <https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust>

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds: Vec<_> = arr.iter().filter(|&x| x % 2 == 1).copied().collect();
    odds.sort_unstable();

    let mut odds = odds.into_iter();
    arr.iter()
        .map(|&x| if x % 2 == 1 { odds.next().unwrap() } else { x })
        .collect()
}
