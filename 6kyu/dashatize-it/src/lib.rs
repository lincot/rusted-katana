//! <https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust>

pub fn dashatize(mut n: i64) -> String {
    if n == 0 {
        return "0".into();
    }
    if n == i64::MIN {
        return "9-22-3-3-7-20-3-68-5-4-7-7-5-808".into();
    }
    if n < 0 {
        n = -n;
    }

    let mut digits = Vec::with_capacity(19);

    while n != 0 {
        digits.push(b'0' + (n % 10) as u8);
        n /= 10;
    }
    if digits.is_empty() {
        unsafe { core::hint::unreachable_unchecked() };
    }

    let mut res = Vec::with_capacity(2 * digits.len());

    let mut digits = digits.into_iter().rev();

    let first = digits.next().unwrap();
    res.push(first);
    let mut was_odd = first % 2 == 1;

    for d in digits {
        let is_odd = d % 2 == 1;

        if was_odd || is_odd {
            res.push(b'-');
        }

        res.push(d);

        was_odd = is_odd;
    }

    unsafe { String::from_utf8_unchecked(res) }
}
