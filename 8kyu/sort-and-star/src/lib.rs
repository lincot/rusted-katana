//! <https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/train/rust>

const STARS: &str = "***";

pub fn two_sort(arr: &[&str]) -> String {
    let min = arr.iter().min().unwrap();
    let mut min_chars = (*min).chars();

    let first = match min_chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    // slower but potentially allocates less
    // let cap = min.len() + STARS.len() * (min.chars().count() - 1);

    let cap = (1 + STARS.len()) * min.len() - STARS.len();
    let mut res = String::with_capacity(cap);

    res.push(first);

    for c in min_chars {
        res.push_str(STARS);
        res.push(c);
    }

    res
}
