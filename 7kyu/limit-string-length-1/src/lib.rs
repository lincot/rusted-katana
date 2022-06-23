//! <https://www.codewars.com/kata/5208fc3cb613bc725f000142/train/rust>

pub fn solution(st: &str, limit: usize) -> String {
    const DOTS: &str = "...";

    // worst case capacity
    let cap = (4 * limit).min(st.len()) + DOTS.len();
    let mut res = String::with_capacity(cap);

    let mut st = st.chars();
    res.extend(st.by_ref().take(limit));

    if st.next().is_some() {
        res.push_str(DOTS);
    }

    res
}
