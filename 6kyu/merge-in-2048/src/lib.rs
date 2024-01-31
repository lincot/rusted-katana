//! <https://www.codewars.com/kata/55e1990978c60e5052000011/train/rust>

pub fn merge(line: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(line.len());
    unsafe { res.set_len(line.len()) };
    let mut i = 0;
    let prev = line.iter().filter(|&&x| x != 0).fold(0, |prev, &x| {
        if prev == x {
            unsafe { *res.get_unchecked_mut(i) = 2 * x };
            i += 1;
            0
        } else if prev != 0 {
            unsafe { *res.get_unchecked_mut(i) = prev };
            i += 1;
            x
        } else {
            x
        }
    });
    if prev != 0 {
        unsafe { *res.get_unchecked_mut(i) = prev };
        i += 1;
    }
    #[allow(clippy::needless_range_loop)]
    for i in i..res.len() {
        res[i] = 0;
    }
    res
}
