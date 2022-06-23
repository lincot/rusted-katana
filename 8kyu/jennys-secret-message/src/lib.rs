//! <https://www.codewars.com/kata/55225023e1be1ec8bc000390/train/rust>

pub fn greet(input: &str) -> String {
    if input == "Johnny" {
        "Hello, my love!".into()
    } else {
        format!("Hello, {}!", input)
    }
}
