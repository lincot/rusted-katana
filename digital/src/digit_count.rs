pub trait DigitCount {
    fn count_digits_base10(self) -> usize;
    fn count_digits_base2(self) -> usize;
    fn count_digits_base16(self) -> usize;
}

trait FastLog: Sized {
    fn fast_log2(self) -> usize;
    fn fast_log10(self) -> usize;
}

macro_rules! impl_fast_log {
    ($($t:ty)*) => ($(
        impl FastLog for $t {
            #[inline]
            fn fast_log2(self) -> usize {
                Self::BITS as usize - 1 - (self | 1).leading_zeros() as usize
            }

            #[inline]
            fn fast_log10(self) -> usize {
                let log2 = self.fast_log2();
                (log2 * 1233) >> 12
            }
        }
    )*)
}

impl_fast_log! { u8 u16 u32 u64 u128 usize }

#[inline]
fn count_digits_base2<T: FastLog>(x: T) -> usize {
    x.fast_log2() + 1
}

#[inline]
fn count_digits_base16<T: FastLog>(x: T) -> usize {
    (count_digits_base2(x) + 3) / 4
}

impl DigitCount for u8 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else {
            3
        }
    }

    #[inline]
    fn count_digits_base2(self) -> usize {
        count_digits_base2(self)
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        count_digits_base16(self)
    }
}

#[inline]
fn fast_digit_count<T: FastLog + Into<u64> + Copy>(x: T) -> usize {
    #[expect(clippy::unreadable_literal)]
    const TABLE: [u64; 32] = [
        4294967296,
        8589934582,
        8589934582,
        8589934582,
        12884901788,
        12884901788,
        12884901788,
        17179868184,
        17179868184,
        17179868184,
        21474826480,
        21474826480,
        21474826480,
        21474826480,
        25769703776,
        25769703776,
        25769703776,
        30063771072,
        30063771072,
        30063771072,
        34349738368,
        34349738368,
        34349738368,
        34349738368,
        38554705664,
        38554705664,
        38554705664,
        41949672960,
        41949672960,
        41949672960,
        42949672960,
        42949672960,
    ];
    let shift = unsafe { TABLE.get_unchecked(x.fast_log2()) };
    let count = (x.into() + shift) >> 32;
    count as _
}

#[inline]
fn fallback_digit_count<T: FastLog + PartialOrd + Copy>(x: T, powers_of_10: &[T]) -> usize {
    let log10 = x.fast_log10();
    let shift_up = powers_of_10.get(log10).is_some_and(|&y| x >= y);
    log10 + shift_up as usize + 1
}

impl DigitCount for u16 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        fast_digit_count(self)
    }

    #[inline]
    fn count_digits_base2(self) -> usize {
        count_digits_base2(self)
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        count_digits_base16(self)
    }
}

impl DigitCount for u32 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        fast_digit_count(self)
    }

    #[inline]
    fn count_digits_base2(self) -> usize {
        count_digits_base2(self)
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        count_digits_base16(self)
    }
}

impl DigitCount for u64 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        const POWERS_OF_10: [u64; 19] = {
            let mut res = [10; 19];
            let mut i = 1;
            while i < res.len() {
                res[i] = 10 * res[i - 1];
                i += 1;
            }
            res
        };
        fallback_digit_count(self, &POWERS_OF_10)
    }

    #[inline]
    fn count_digits_base2(self) -> usize {
        count_digits_base2(self)
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        count_digits_base16(self)
    }
}

impl DigitCount for usize {
    #[inline]
    fn count_digits_base10(self) -> usize {
        #[cfg(target_pointer_width = "16")]
        {
            (self as u16).count_digits_base10()
        }
        #[cfg(target_pointer_width = "32")]
        {
            (self as u32).count_digits_base10()
        }
        #[cfg(target_pointer_width = "64")]
        {
            (self as u64).count_digits_base10()
        }
    }

    #[inline]
    fn count_digits_base2(self) -> usize {
        count_digits_base2(self)
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        count_digits_base16(self)
    }
}

impl DigitCount for u128 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        const POWERS_OF_10: [u128; 38] = {
            let mut res = [10; 38];
            let mut i = 1;
            while i < res.len() {
                res[i] = 10 * res[i - 1];
                i += 1;
            }
            res
        };
        fallback_digit_count(self, &POWERS_OF_10)
    }

    #[inline]
    fn count_digits_base2(self) -> usize {
        count_digits_base2(self)
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        count_digits_base16(self)
    }
}
