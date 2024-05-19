//! <https://www.codewars.com/kata/5ba178be875de960a6000187/train/rust>

pub fn find_lowest_int(k: u64) -> u64 {
    match k % 100 {
        5 | 21 | 26 | 42 | 47 | 63 | 68 | 84 | 89 => return 9,
        18 | 39 | 76 | 97 => return 18,
        54 | 61 | 75 | 82 => return 27,
        14 | 43 | 64 | 93 => return 36,
        9 | 29 | 49 | 69 => return 45,
        32 => return 54,
        83 | 86 => return 63,
        _ => {}
    }
    let mut n = 27;
    loop {
        if digits_hash(n * k) == digits_hash(n * k + n) {
            return n;
        }
        n += 9;
    }
}

fn digits_hash(mut n: u64) -> u64 {
    let mut res = 0;
    while n != 0 {
        res += match n % 10 {
            0 => 21u64.pow(0),
            1 => 21u64.pow(1),
            2 => 21u64.pow(2),
            3 => 21u64.pow(3),
            4 => 21u64.pow(4),
            5 => 21u64.pow(5),
            6 => 21u64.pow(6),
            7 => 21u64.pow(7),
            8 => 21u64.pow(8),
            9 => 21u64.pow(9),
            _ => unreachable!(),
        };
        n /= 10;
    }
    res
}
