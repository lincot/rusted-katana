pub trait DigitSum {
    fn sum_digits(self) -> u32;
}

const DIGIT_SUM_BY_PAIR: [u8; 100] = {
    let mut table = [0; 100];
    let mut i = 0;
    while i < 100 {
        table[i] = (i / 10 + i % 10) as u8;
        i += 1;
    }
    table
};

macro_rules! impl_digit_sum {
    ($($t:ty)*) => ($(
        impl DigitSum for $t {
            #[inline]
            fn sum_digits(mut self) -> u32 {
                let mut res = 0;
                while self >= 10 {
                    res += DIGIT_SUM_BY_PAIR[(self % 100) as usize] as u32;
                    self /= 100;
                }
                res += self as u32;
                res
            }
        }
    )*)
}

impl_digit_sum! { u8 u16 u32 u64 usize }
