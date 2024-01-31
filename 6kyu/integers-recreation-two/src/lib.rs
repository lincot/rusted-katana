//! <https://www.codewars.com/kata/55e86e212fce2aae75000060/train/rust>

pub fn prod2sum(a: i64, b: i64, c: i64, d: i64) -> Vec<(i64, i64)> {
    let e0 = (a * c + b * d).abs();
    let f0 = (a * d - b * c).abs();
    let e1 = (a * c - b * d).abs();
    let f1 = (a * d + b * c).abs();

    let first = (e0.min(f0), e0.max(f0));
    let second = (e1.min(f1), e1.max(f1));

    if first == second {
        vec![first]
    } else {
        vec![first.min(second), first.max(second)]
    }
}
