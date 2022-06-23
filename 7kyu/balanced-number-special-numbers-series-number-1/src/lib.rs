//! <https://www.codewars.com/kata/5a4e3782880385ba68000018/train/rust>

fn u64_to_digits(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::with_capacity(20);

    while n != 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    digits
}

pub fn balanced_num(n: u64) -> String {
    let digits = u64_to_digits(n);

    if unsafe { digits.get_unchecked(0..(digits.len() - 1) / 2) }
        .iter()
        .sum::<u8>()
        == unsafe { digits.get_unchecked(digits.len() / 2 + 1..) }
            .iter()
            .sum()
    {
        "Balanced"
    } else {
        "Not Balanced"
    }
    .into()
}
