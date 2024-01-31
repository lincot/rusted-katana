//! <https://www.codewars.com/kata/5772da22b89313a4d50012f7/train/rust>

pub fn greet(name: &str, owner: &str) -> String {
    if name == owner {
        "Hello boss".into()
    } else {
        "Hello guest".into()
    }
}
