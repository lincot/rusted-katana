//! <https://www.codewars.com/kata/5edc8c53d7cede0032eb6029/train/rust>

pub fn solve(n: u64) -> Option<u64> {
    if n == 0 {
        Some(1)
    } else if n % 4 == 2 {
        None
    } else if n.is_multiple_of(4) {
        let m = n / 4;
        (1u64..unsafe { ((m as f64).sqrt() + 1.5).to_int_unchecked() })
            .rev()
            .skip_while(|b| m <= b.pow(2))
            .find(|&b| m.is_multiple_of(b))
            .map(|b| {
                let a = m / b;
                let x = a - b;
                x.pow(2)
            })
    } else {
        (1..unsafe { ((n as f64).sqrt() + 2.5).to_int_unchecked::<u64>() } / 2)
            .rev()
            .map(|i| 2 * i - 1)
            .skip_while(|b| n <= b.pow(2))
            .find(|&b| n.is_multiple_of(b))
            .map(|b| {
                let a = n / b;
                let x = (a - b) / 2;
                x.pow(2)
            })
    }
}
