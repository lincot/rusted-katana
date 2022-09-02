//! <https://www.codewars.com/kata/5b728f801db5cec7320000c7/train/rust>

pub fn solve(s: &str, k: usize) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    unsafe { bytes.set_len(s.len()) };
    let mut bytes_ptr = bytes.as_mut_ptr();
    for p in s.bytes().enumerate() {
        unsafe {
            *bytes_ptr = p;
            bytes_ptr = bytes_ptr.add(1);
        }
    }
    bytes.sort_by_key(|&(_, b)| b);
    bytes.drain(0..bytes.len().min(k));
    bytes.sort_unstable_by_key(|&(i, _)| i);
    let mut res = Vec::with_capacity(bytes.len());
    unsafe { res.set_len(bytes.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for (_, b) in bytes {
        unsafe {
            assert!(b.is_ascii());
            *res_ptr = b;
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
