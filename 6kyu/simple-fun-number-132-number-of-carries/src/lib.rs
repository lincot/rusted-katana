//! <https://www.codewars.com/kata/58a6568827f9546931000027/train/rust>

pub const fn number_of_carries(mut a: u32, mut b: u32) -> usize {
    let mut res = 0;
    let mut c = 0;
    while a != 0 || b != 0 {
        let da = a % 10;
        a /= 10;
        let db = b % 10;
        b /= 10;
        c += da + db;
        c /= 10;
        if c != 0 {
            res += 1;
        }
    }
    res
}
