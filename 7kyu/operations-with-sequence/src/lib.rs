//! <https://www.codewars.com/kata/596ddaccdd42c1cf0e00005c/train/rust>

pub fn calc(mut array: Vec<i32>) -> i32 {
    let mut res = 0;

    for x in &mut array {
        if *x > 0 {
            *x *= *x;
        }

        res += *x;
    }

    let mut i = 4;

    while i < array.len() {
        res -= 2 * array[i];
        i += 5;
    }

    i = 2;

    while i < array.len() {
        res += 2 * array[i];
        i += 3;
    }

    i = 14;

    while i < array.len() {
        res -= 4 * array[i];
        i += 15;
    }

    res
}
