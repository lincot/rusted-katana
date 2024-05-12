//! <https://www.codewars.com/kata/5713338968e0cf779b00096e/train/rust>

pub fn find_comb_noncontig(arr: &[i32], t: i32, k: u32) -> u32 {
    let mut res = 0;
    if arr
        .first()
        .is_some_and(|x| (t - k as i32..=t + k as i32).contains(x))
    {
        res += 1;
    }
    if arr.len() > 1 {
        res += find_comb_noncontig(&arr[1..], t, k) + find_comb_noncontig(&arr[2..], t - arr[0], k);
    }
    res
}
