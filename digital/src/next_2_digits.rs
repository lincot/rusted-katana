use super::{digit::Digit, format::Next2DigitsFormat};

pub trait Next2Digits {
    fn next_2_digits<F: Next2DigitsFormat>(&mut self) -> Option<[u8; 2]>;
}

macro_rules! impl_next_2_digits {
    ($($t:ty)*) => ($(
        impl Next2Digits for $t {
            #[inline]
            fn next_2_digits<F: Next2DigitsFormat>(&mut self) -> Option<[u8; 2]> {
                if *self < 10 {
                    return None;
                }

                let to_base10 = if F::RAW {
                    u8::TO_BASE10_RAW
                } else {
                    u8::TO_BASE10
                };
                let res = [
                    to_base10[(*self % 100) as usize * 2],
                    to_base10[(*self % 100) as usize * 2 + 1],
                ];
                *self /= 100;

                if F::RAW && (res[0] > 9 || res[1] > 9)
                    || !F::RAW && (res[0] > b'9' || res[1] > b'9')
                {
                    unsafe { core::hint::unreachable_unchecked() };
                }

                Some(res)
            }
        }
    )*)
}

impl_next_2_digits! { u8 u16 u32 u64 u128 usize }
