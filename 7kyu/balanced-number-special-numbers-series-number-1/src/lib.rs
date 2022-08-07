//! <https://www.codewars.com/kata/5a4e3782880385ba68000018/train/rust>

pub fn balanced_num(n: u64) -> String {
    fn to_digits(mut n: u64) -> ([u8; 20], usize) {
        let (mut digits, mut len) = ([0; 20], 0);
        while n != 0 {
            unsafe {
                *digits.get_unchecked_mut(len) = (n % 10) as u8;
            }
            n /= 10;
            len += 1;
        }
        (digits, len)
    }

    if n == 0 {
        return "Balanced".into();
    }
    let (digits, len) = to_digits(n);
    if unsafe { digits.get_unchecked(0..((len - 1) / 2)) }
        .iter()
        .sum::<u8>()
        == unsafe { digits.get_unchecked((len / 2 + 1)..len) }
            .iter()
            .sum()
    {
        "Balanced".into()
    } else {
        "Not Balanced".into()
    }
}
