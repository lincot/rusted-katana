//! <https://www.codewars.com/kata/56971747aa359bdbf800004d/train/rust>

pub const fn tricky_doubles(num: u32) -> u32 {
    let number_of_digits = if num > 0 { num.ilog10() + 1 } else { return 0 };
    if number_of_digits % 2 == 1 {
        return 2 * num;
    }
    let divisor = POWERS_OF_10[(number_of_digits / 2) as usize];
    if num / divisor == num % divisor {
        num
    } else {
        2 * num
    }
}

const POWERS_OF_10: [u32; 6] = {
    let mut res = [1; 6];
    let mut i = 1;
    while i < res.len() {
        res[i] = 10 * res[i - 1];
        i += 1;
    }
    res
};
