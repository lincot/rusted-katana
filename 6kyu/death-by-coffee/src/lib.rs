//! <https://www.codewars.com/kata/57db78d3b43dfab59c001abe/train/rust>

pub fn coffee_limits(y: u32, m: u32, d: u32) -> (u32, u32) {
    let h = (y * 10000 + m * 100 + d) as u64;
    (coffee_limit(h, 0xcafe), coffee_limit(h, 0xdecaf))
}

fn coffee_limit(h: u64, coffee_type: u64) -> u32 {
    (1u32..=5000)
        .find(|&i| {
            let v = h + coffee_type * i as u64;
            (0..6).any(|r| v >> (4 * r) & 0xffff == 0xdead)
        })
        .unwrap_or(0)
}
