//! <https://www.codewars.com/kata/57ea5b0b75ae11d1e800006c/train/rust>

pub fn sort_by_length(arr: &[String]) -> Vec<String> {
    let mut res = arr.to_vec();
    if res.len() < 256 {
        res.sort_unstable_by_key(String::len);
    } else {
        radsort::sort_by_key(&mut res, String::len);
    }
    res
}
