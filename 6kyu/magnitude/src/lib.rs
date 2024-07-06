//! <https://www.codewars.com/kata/5cc70653658d6f002ab170b5/train/rust>

pub struct S {
    pub s: String,
    pub xs: Vec<i32>,
}

pub fn sqr_modulus(a: S) -> (bool, i32, i32) {
    match a.s.as_str() {
        "cart" => {
            let sum = a.xs.iter().map(|x| x * x).sum();
            (true, sum, descending_order(sum))
        }
        "polar" => {
            let sum = a.xs.chunks(2).map(|pair| pair[0] * pair[0]).sum();
            (true, sum, descending_order(sum))
        }
        _ => (false, -1, 1),
    }
}

fn descending_order(mut x: i32) -> i32 {
    let mut digits = [0u8; 10];

    while x != 0 {
        digits[(x % 10) as usize] += 1;
        x /= 10;
    }

    for (i, &n) in digits.iter().enumerate().rev() {
        let (a, b) = unsafe { TABLE.get_unchecked(n as usize) };
        x = x * a + b * i as i32;
    }

    x
}

const TABLE: [(i32, i32); 11] = [
    (1, 0),
    (10, 1),
    (100, 11),
    (1_000, 111),
    (10_000, 1_111),
    (100_000, 11_111),
    (1_000_000, 111_111),
    (10_000_000, 1_111_111),
    (100_000_000, 11_111_111),
    (1_000_000_000, 111_111_111),
    (1_410_065_408, 1_111_111_111),
];
