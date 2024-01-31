//! <https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/train/rust>

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.iter()
        .map(|&(age, handicap)| {
            if age >= 55 && handicap > 7 {
                "Senior".to_string()
            } else {
                "Open".to_string()
            }
        })
        .collect()
}
