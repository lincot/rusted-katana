//! <https://www.codewars.com/kata/5a905c2157c562994900009d/train/rust>

pub fn product_array(arr: &[u64]) -> Vec<u64> {
    let prod: u64 = arr.iter().product();
    assert!(prod != 0);
    let mut res = Vec::with_capacity(arr.len());
    unsafe { res.set_len(arr.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &x in arr {
        if x == 0 {
            unsafe { core::hint::unreachable_unchecked() };
        }
        unsafe {
            *res_ptr = prod / x;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
