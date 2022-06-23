//! <https://www.codewars.com/kata/5d23d89906f92a00267bb83d/train/rust>

pub fn get_order(input: String) -> String {
    let cap = input.len() + input.len() / 4;
    let mut res = String::with_capacity(cap);

    let mut p = 0usize;
    let mut m = 0;
    let mut n = 0;
    let mut f = 0;
    let mut g = 0;
    let mut o = 0;
    let mut u = 0;
    let mut w = 0;

    for b in input.bytes() {
        match b {
            b'p' => p += 1,
            b'm' => m += 1,
            b'n' => n += 1,
            b'f' => f += 1,
            b'g' => g += 1,
            b'o' => o += 1,
            b'u' => u += 1,
            b'w' => w += 1,
            _ => {}
        }
    }

    for _ in 0..u {
        res.push_str("Burger ");
    }
    for _ in 0..f {
        res.push_str("Fries ");
    }
    for _ in 0..n - 3 * (g - u) - w {
        res.push_str("Chicken ");
    }
    for _ in 0..p {
        res.push_str("Pizza ");
    }
    for _ in 0..w {
        res.push_str("Sandwich ");
    }
    for _ in 0..g - u {
        res.push_str("Onionrings ");
    }
    for _ in 0..m {
        res.push_str("Milkshake ");
    }
    for _ in 0..o - 2 * (g - u) {
        res.push_str("Coke ");
    }
    res.pop();

    res
}
