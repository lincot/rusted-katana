//! <https://www.codewars.com/kata/59530d2401d6039f8600001f/train/rust>

pub fn how_many_measurements(n: u64) -> u32 {
    const POWERS_OF_3: [u64; 41] = {
        let mut res = [1; 41];
        let mut i = 1;
        while i < res.len() {
            res[i] = 3 * res[i - 1];
            i += 1;
        }
        res
    };
    POWERS_OF_3.binary_search(&n).unwrap_or_else(|x| x) as _
}
