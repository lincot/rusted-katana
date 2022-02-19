//! <https://www.codewars.com/kata/5b728f801db5cec7320000c7/train/rust>

pub fn solve(s: &str, k: usize) -> String {
    let mut bytes: Vec<_> = s.bytes().enumerate().collect();
    bytes.sort_by_key(|&(_, b)| b);

    let mut bytes: Vec<_> = bytes.into_iter().skip(k).collect();
    bytes.sort_unstable_by_key(|&(i, _)| i);

    String::from_utf8(bytes.into_iter().map(|(_, b)| b).collect()).unwrap()
}
