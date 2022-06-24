//! <https://www.codewars.com/kata/545af3d185166a3dec001190/train/rust>

pub fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    arr.windows(n).map(Vec::from).collect()
}
