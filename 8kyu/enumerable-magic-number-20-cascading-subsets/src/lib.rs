//! <https://www.codewars.com/kata/545af3d185166a3dec001190/train/rust>

pub fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    let len = if n == 0 || arr.len() < n {
        0
    } else {
        arr.len() - n + 1
    };
    (0..len)
        .map(|i| unsafe { arr.get_unchecked(i..i + n).to_vec() })
        .collect()
}
