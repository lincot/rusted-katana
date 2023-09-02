//! fast integer to string/array of digits
//! based on [rust-lexical](https://github.com/Alexhuszagh/rust-lexical)
#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

trait PushUnchecked<T> {
    /// # Safety
    ///
    /// `self.len()` must be `< self.capacity()`
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn push_unchecked(&mut self, value: T) {
        debug_assert!(self.len() < self.capacity());
        if self.len() >= self.capacity() {
            core::hint::unreachable_unchecked();
        }
        self.push(value);
    }
}

#[inline]
unsafe fn write_digit<T: From<u8>>(
    buffer: &mut [T],
    index: &mut usize,
    r: u8,
    radix: u8,
    reversed: bool,
    from_0: bool,
) {
    if !reversed {
        *index -= 1;
    }
    *buffer.get_unchecked_mut(*index) = (if from_0 {
        r
    } else if radix != 16 {
        r + b'0'
    } else {
        *[
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd',
            b'e', b'f',
        ]
        .get_unchecked(r as usize)
    })
    .into();
    if reversed {
        *index = index.wrapping_add(1);
    }
}

trait TabledInteger: Sized + Copy + From<u8> + 'static {
    const TO_BASE2: &'static [Self; 8];
    const TO_BASE2_FROM_0: &'static [Self; 8];
    const TO_BASE2_REVERSED_FROM_0: &'static [Self; 8];
    const TO_BASE2_REVERSED: &'static [Self; 8];
    const TO_BASE10: &'static [Self; 200];
    const TO_BASE10_FROM_0: &'static [Self; 200];
    const TO_BASE10_REVERSED_FROM_0: &'static [Self; 200];
    const TO_BASE10_REVERSED: &'static [Self; 200];
    const TO_BASE16: &'static [Self; 512];
    const TO_BASE16_REVERSED: &'static [Self; 512];
}

macro_rules! impl_tabled_integer {
    ($($t:ty)*) => ($(
        impl TabledInteger for $t {
            const TO_BASE10_FROM_0: &'static [Self; 200] = &[
                0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9,
                1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9,
                2, 0, 2, 1, 2, 2, 2, 3, 2, 4, 2, 5, 2, 6, 2, 7, 2, 8, 2, 9,
                3, 0, 3, 1, 3, 2, 3, 3, 3, 4, 3, 5, 3, 6, 3, 7, 3, 8, 3, 9,
                4, 0, 4, 1, 4, 2, 4, 3, 4, 4, 4, 5, 4, 6, 4, 7, 4, 8, 4, 9,
                5, 0, 5, 1, 5, 2, 5, 3, 5, 4, 5, 5, 5, 6, 5, 7, 5, 8, 5, 9,
                6, 0, 6, 1, 6, 2, 6, 3, 6, 4, 6, 5, 6, 6, 6, 7, 6, 8, 6, 9,
                7, 0, 7, 1, 7, 2, 7, 3, 7, 4, 7, 5, 7, 6, 7, 7, 7, 8, 7, 9,
                8, 0, 8, 1, 8, 2, 8, 3, 8, 4, 8, 5, 8, 6, 8, 7, 8, 8, 8, 9,
                9, 0, 9, 1, 9, 2, 9, 3, 9, 4, 9, 5, 9, 6, 9, 7, 9, 8, 9, 9,
            ];

            const TO_BASE10: &'static [Self; 200] = &{
                let mut arr = [0; 200];
                let mut i = 0;
                while i < arr.len() {
                    arr[i] = Self::TO_BASE10_FROM_0[i] + b'0' as Self;
                    i += 1;
                }
                arr
            };

            const TO_BASE10_REVERSED_FROM_0: &'static [Self; 200] = &{
                let mut res = [0; 200];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE10_FROM_0[i + 1];
                    res[i + 1] = Self::TO_BASE10_FROM_0[i];
                    i += 2;
                }
                res
            };

            const TO_BASE10_REVERSED: &'static [Self; 200] = &{
                let mut arr = [0; 200];
                let mut i = 0;
                while i < arr.len() {
                    arr[i] = Self::TO_BASE10_REVERSED_FROM_0[i] + b'0' as Self;
                    i += 1;
                }
                arr
            };

            const TO_BASE2_FROM_0: &'static [Self; 8] = &[0, 0, 0, 1, 1, 0, 1, 1];

            const TO_BASE2: &'static [Self; 8] = &{
                let mut arr = [0; 8];
                let mut i = 0;
                while i < arr.len() {
                    arr[i] += Self::TO_BASE2_FROM_0[i] + b'0' as Self;
                    i += 1;
                }
                arr
            };

            const TO_BASE2_REVERSED_FROM_0: &'static [Self; 8] = &{
                let mut res = [0; 8];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE2_FROM_0[i + 1];
                    res[i + 1] = Self::TO_BASE2_FROM_0[i];
                    i += 2;
                }
                res
            };

            const TO_BASE2_REVERSED: &'static [Self; 8] = &{
                let mut arr = [0; 8];
                let mut i = 0;
                while i < arr.len() {
                    arr[i] = Self::TO_BASE2_REVERSED_FROM_0[i] + b'0' as Self;
                    i += 1;
                }
                arr
            };


            const TO_BASE16: &'static [Self; 512] = &{
                const TO_BASE16_U8: [u8; 512] = [
                    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
                    b'0', b'8', b'0', b'9', b'0', b'a', b'0', b'b', b'0', b'c', b'0', b'd', b'0', b'e', b'0', b'f',
                    b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5', b'1', b'6', b'1', b'7',
                    b'1', b'8', b'1', b'9', b'1', b'a', b'1', b'b', b'1', b'c', b'1', b'd', b'1', b'e', b'1', b'f',
                    b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7',
                    b'2', b'8', b'2', b'9', b'2', b'a', b'2', b'b', b'2', b'c', b'2', b'd', b'2', b'e', b'2', b'f',
                    b'3', b'0', b'3', b'1', b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7',
                    b'3', b'8', b'3', b'9', b'3', b'a', b'3', b'b', b'3', b'c', b'3', b'd', b'3', b'e', b'3', b'f',
                    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
                    b'4', b'8', b'4', b'9', b'4', b'a', b'4', b'b', b'4', b'c', b'4', b'd', b'4', b'e', b'4', b'f',
                    b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5', b'5', b'6', b'5', b'7',
                    b'5', b'8', b'5', b'9', b'5', b'a', b'5', b'b', b'5', b'c', b'5', b'd', b'5', b'e', b'5', b'f',
                    b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3', b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7',
                    b'6', b'8', b'6', b'9', b'6', b'a', b'6', b'b', b'6', b'c', b'6', b'd', b'6', b'e', b'6', b'f',
                    b'7', b'0', b'7', b'1', b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7',
                    b'7', b'8', b'7', b'9', b'7', b'a', b'7', b'b', b'7', b'c', b'7', b'd', b'7', b'e', b'7', b'f',
                    b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8', b'5', b'8', b'6', b'8', b'7',
                    b'8', b'8', b'8', b'9', b'8', b'a', b'8', b'b', b'8', b'c', b'8', b'd', b'8', b'e', b'8', b'f',
                    b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4', b'9', b'5', b'9', b'6', b'9', b'7',
                    b'9', b'8', b'9', b'9', b'9', b'a', b'9', b'b', b'9', b'c', b'9', b'd', b'9', b'e', b'9', b'f',
                    b'a', b'0', b'a', b'1', b'a', b'2', b'a', b'3', b'a', b'4', b'a', b'5', b'a', b'6', b'a', b'7',
                    b'a', b'8', b'a', b'9', b'a', b'a', b'a', b'b', b'a', b'c', b'a', b'd', b'a', b'e', b'a', b'f',
                    b'b', b'0', b'b', b'1', b'b', b'2', b'b', b'3', b'b', b'4', b'b', b'5', b'b', b'6', b'b', b'7',
                    b'b', b'8', b'b', b'9', b'b', b'a', b'b', b'b', b'b', b'c', b'b', b'd', b'b', b'e', b'b', b'f',
                    b'c', b'0', b'c', b'1', b'c', b'2', b'c', b'3', b'c', b'4', b'c', b'5', b'c', b'6', b'c', b'7',
                    b'c', b'8', b'c', b'9', b'c', b'a', b'c', b'b', b'c', b'c', b'c', b'd', b'c', b'e', b'c', b'f',
                    b'd', b'0', b'd', b'1', b'd', b'2', b'd', b'3', b'd', b'4', b'd', b'5', b'd', b'6', b'd', b'7',
                    b'd', b'8', b'd', b'9', b'd', b'a', b'd', b'b', b'd', b'c', b'd', b'd', b'd', b'e', b'd', b'f',
                    b'e', b'0', b'e', b'1', b'e', b'2', b'e', b'3', b'e', b'4', b'e', b'5', b'e', b'6', b'e', b'7',
                    b'e', b'8', b'e', b'9', b'e', b'a', b'e', b'b', b'e', b'c', b'e', b'd', b'e', b'e', b'e', b'f',
                    b'f', b'0', b'f', b'1', b'f', b'2', b'f', b'3', b'f', b'4', b'f', b'5', b'f', b'6', b'f', b'7',
                    b'f', b'8', b'f', b'9', b'f', b'a', b'f', b'b', b'f', b'c', b'f', b'd', b'f', b'e', b'f', b'f',
                ];
                let mut res = [0; 512];
                let mut i = 0;
                while i < res.len() {
                    res[i] = TO_BASE16_U8[i] as Self;
                    i += 1;
                }
                res
            };

            const TO_BASE16_REVERSED: &'static [Self; 512] = &{
                let mut res = [0; 512];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE16[i + 1];
                    res[i + 1] = Self::TO_BASE16[i];
                    i += 2;
                }
                res
            };
        }
    )*)
}

impl_tabled_integer! { u8 u16 i16 u32 i32 u64 i64 usize isize }

#[inline]
unsafe fn write_2_digits<T: TabledInteger>(
    buffer: &mut [T],
    index: &mut usize,
    r: usize,
    reversed: bool,
    table: &[T],
) {
    if reversed {
        buffer
            .get_unchecked_mut(*index..(*index).wrapping_add(2))
            .copy_from_slice(table.get_unchecked(r..r + 2));
        *index = (*index).wrapping_add(2);
    } else {
        buffer
            .get_unchecked_mut(*index - 2..*index)
            .copy_from_slice(table.get_unchecked(r..r + 2));
        *index -= 2;
    }
}

trait WriteDigits {
    unsafe fn write_digits<T: TabledInteger>(
        self,
        radix: u8,
        buffer: &mut [T],
        index: usize,
        reversed: bool,
        from_0: bool,
    ) -> usize;
}

trait LogAndCount: Sized {
    fn fast_log2(self) -> usize;
    fn fast_log10(self) -> usize;
    fn fast_digit_count(self) -> usize;
    fn fallback_digit_count(self, table: &[Self]) -> usize;
}

pub trait CountDigitsBase10 {
    fn count_digits_base10(self) -> usize;
}

pub trait CountDigitsBase2And16 {
    fn count_digits_base2(self) -> usize;
    fn count_digits_base16(self) -> usize;
}

impl<T: LogAndCount> CountDigitsBase2And16 for T {
    #[inline]
    fn count_digits_base2(self) -> usize {
        self.fast_log2() + 1
    }

    #[inline]
    fn count_digits_base16(self) -> usize {
        (self.count_digits_base2() + 3) / 4
    }
}

pub trait WriteNumUnchecked<T> {
    /// # Safety
    ///
    /// `self.len() + n.to_string().len()` must be `<= self.capacity()`
    unsafe fn write_num_unchecked(&mut self, n: T, radix: u8, reversed: bool, from_0: bool);
}

macro_rules! impl_log_and_count {
    ($($t:ty)*) => ($(
        impl LogAndCount for $t {
            #[inline]
            fn fast_log2(self) -> usize {
                Self::BITS as usize - 1 - (self | 1).leading_zeros() as usize
            }

            #[inline]
            fn fast_log10(self) -> usize {
                let log2 = self.fast_log2();
                (log2 * 1233) >> 12
            }

            #[inline]
            fn fast_digit_count(self) -> usize {
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
                let shift = unsafe { TABLE.get_unchecked(self.fast_log2()) };
                let count = (self as u64 + shift) >> 32;
                count as _
            }

            #[inline]
            fn fallback_digit_count(self: $t, table: &[$t]) -> usize {
                let log10 = self.fast_log10();
                let shift_up = table.get(log10).map_or(false, |&y| self >= y);
                log10 + shift_up as usize + 1
            }
        }
    )*)
}

impl_log_and_count! { u8 u16 u32 u64 u128 usize }

impl CountDigitsBase10 for u8 {
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
}

impl CountDigitsBase10 for u16 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        self.fast_digit_count()
    }
}

impl CountDigitsBase10 for u32 {
    #[inline]
    fn count_digits_base10(self) -> usize {
        self.fast_digit_count()
    }
}

impl CountDigitsBase10 for u64 {
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
        self.fallback_digit_count(&POWERS_OF_10)
    }
}

impl CountDigitsBase10 for usize {
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
}

impl CountDigitsBase10 for u128 {
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
        self.fallback_digit_count(&POWERS_OF_10)
    }
}

macro_rules! gen_write_num_unchecked_unsigned {
    ($t:ty) => {
        #[inline]
        unsafe fn write_num_unchecked(&mut self, n: $t, radix: u8, reversed: bool, from_0: bool) {
            let len = self.len();
            let index = if reversed {
                0
            } else {
                match radix {
                    2 => n.count_digits_base2(),
                    10 => {
                        if <$t>::BITS == 128 && u64::try_from(n).is_ok() {
                            (n as u64).count_digits_base10()
                        } else {
                            n.count_digits_base10()
                        }
                    }
                    16 => n.count_digits_base16(),
                    _ => unimplemented!(),
                }
            };
            let index2 = n.write_digits(
                radix,
                core::slice::from_raw_parts_mut(self.as_mut_ptr().add(len), self.capacity() - len),
                index,
                reversed,
                from_0,
            );
            self.set_len(len + if reversed { index2 } else { index });
        }
    };
}

macro_rules! impl_write_digits {
    ($($t:ty)*) => ($(
        impl WriteDigits for $t {
            #[inline]
            unsafe fn write_digits<T: TabledInteger>(
                mut self,
                radix: u8,
                buffer: &mut [T],
                mut index: usize,
                reversed: bool,
                from_0: bool,
            ) -> usize {
                let table: &[T] = match radix {
                    2 => {
                        if reversed {
                            if from_0 {
                                T::TO_BASE2_REVERSED_FROM_0
                            } else {
                                T::TO_BASE2_REVERSED
                            }
                        } else if from_0 {
                            T::TO_BASE2_FROM_0
                        } else {
                            T::TO_BASE2
                        }
                    },
                    10 => {
                        if reversed {
                            if from_0 {
                                T::TO_BASE10_REVERSED_FROM_0
                            } else {
                                T::TO_BASE10_REVERSED
                            }
                        } else if from_0 {
                            T::TO_BASE10_FROM_0
                        } else {
                            T::TO_BASE10
                        }
                    },
                    16 => {
                        if from_0 {
                            unimplemented!()
                        } else if reversed {
                            T::TO_BASE16_REVERSED
                        } else {
                            T::TO_BASE16
                        }
                    },
                    _ => unimplemented!(),
                };

                let radix = radix as $t;
                if let Some(radix2) = radix.checked_mul(radix) {
                    if let Some(radix4) = radix2.checked_mul(radix2) {
                        while self >= radix4 {
                            let r = self % radix4;
                            self /= radix4;
                            let r1 = (2 * (r / radix2)) as usize;
                            let r2 = (2 * (r % radix2)) as usize;

                            write_2_digits(buffer, &mut index, r2, reversed, table);
                            write_2_digits(buffer, &mut index, r1, reversed, table);
                        }
                    }

                    while self >= radix2 {
                        let r = (2 * (self % radix2)) as usize;
                        self /= radix2;

                        write_2_digits(buffer, &mut index, r, reversed, table);
                    }
                }

                if self < radix {
                    let r = self as _;
                    write_digit(buffer, &mut index, r, radix as u8, reversed, from_0);
                } else {
                    let r = (2 * self) as usize;
                    write_2_digits(buffer, &mut index, r, reversed, table);
                }

                index
            }
        }
    )*)
}

impl_write_digits! { u8 u16 u32 u64 usize }

macro_rules! impl_write_num_unchecked_unsigned {
    ($($t:ty)*) => ($(
        impl<T: TabledInteger> WriteNumUnchecked<$t> for Vec<T> {
            gen_write_num_unchecked_unsigned!($t);
        }

        #[cfg(feature = "heapless")]
        impl<T: TabledInteger, const N: usize> WriteNumUnchecked<$t> for heapless::Vec<T, N> {
            gen_write_num_unchecked_unsigned!($t);
        }
    )*)
}

impl_write_num_unchecked_unsigned! { u8 u16 u32 u64 u128 usize }

#[inline]
const fn mulhi_u128(x: u128, y: u128) -> u128 {
    let x1 = x >> u64::BITS as i32;
    let x0 = x & u64::MAX as u128;
    let y1 = y >> u64::BITS as i32;
    let y0 = y & u64::MAX as u128;

    let w0 = x0 * y0;
    let m = (x0 * y1) + (w0 >> u64::BITS as i32);
    let w1 = m & u64::MAX as u128;
    let w2 = m >> u64::BITS as i32;

    let w3 = (x1 * y0 + w1) >> u64::BITS as i32;

    x1 * y1 + w2 + w3
}

#[inline]
const fn fast_u128_divrem(
    n: u128,
    d: u64,
    fast: u128,
    fast_shr: u32,
    factor: u128,
    factor_shr: u32,
) -> (u128, u64) {
    let quot = if n < fast {
        ((n >> fast_shr) as u64 / (d >> fast_shr)) as u128
    } else {
        mulhi_u128(n, factor) >> factor_shr
    };
    let rem = (n - quot * d as u128) as u64;
    (quot, rem)
}

#[inline]
fn u128_divrem(n: u128, radix: u8) -> (u128, u64) {
    match radix {
        2 | 16 => (n >> 64, n as u64),
        10 => fast_u128_divrem(
            n,
            10_000_000_000_000_000_000,
            9_671_406_556_917_033_397_649_408,
            19,
            156_927_543_384_667_019_095_894_735_580_191_660_403,
            62,
        ),
        _ => unimplemented!(),
    }
}

#[inline]
unsafe fn write_step_digits<T: TabledInteger>(
    value: u64,
    radix: u8,
    buffer: &mut [T],
    index: usize,
    step: usize,
    reversed: bool,
    from_0: bool,
) -> usize {
    let start = index;
    let index = value.write_digits(radix, buffer, index, reversed, from_0);
    let (end, zeros) = if reversed {
        let end = start.wrapping_add(step);
        let zeros = buffer.get_unchecked_mut(index..end);
        (end, zeros)
    } else {
        let end = start.saturating_sub(step);
        let zeros = buffer.get_unchecked_mut(end..index);
        (end, zeros)
    };
    core::ptr::write_bytes(
        zeros.as_mut_ptr(),
        if from_0 { 0 } else { b'0' },
        zeros.len(),
    );
    end
}

impl WriteDigits for u128 {
    #[inline]
    unsafe fn write_digits<T: TabledInteger>(
        self,
        radix: u8,
        buffer: &mut [T],
        mut index: usize,
        reversed: bool,
        from_0: bool,
    ) -> usize {
        if let Ok(value) = TryInto::<u64>::try_into(self) {
            return value.write_digits(radix, buffer, index, reversed, from_0);
        }

        let step = match radix {
            2 => 64,
            10 => 19,
            16 => 16,
            _ => unimplemented!(),
        };
        let (value, low) = u128_divrem(self, radix);
        index = write_step_digits(low, radix, buffer, index, step, reversed, from_0);
        if let Ok(value) = TryInto::<u64>::try_into(self) {
            return value.write_digits(radix, buffer, index, reversed, from_0);
        }

        let (value, mid) = u128_divrem(value, radix);
        index = write_step_digits(mid, radix, buffer, index, step, reversed, from_0);
        if index != 0 {
            index = (value as u64).write_digits(radix, buffer, index, reversed, from_0);
        }

        index
    }
}

macro_rules! gen_write_num_unchecked_signed {
    ($t:ty) => {
        #[inline]
        unsafe fn write_num_unchecked(&mut self, n: $t, radix: u8, reversed: bool, from_0: bool) {
            if n < 0 {
                self.push_unchecked(b'-'.into());
            }
            self.write_num_unchecked(n.unsigned_abs(), radix, reversed, from_0);
        }
    };
}

macro_rules! impl_write_num_unchecked_signed {
    ($($t:ty)*) => ($(
        impl<T: TabledInteger> WriteNumUnchecked<$t> for Vec<T> {
            gen_write_num_unchecked_signed!($t);
        }

        #[cfg(feature = "heapless")]
        impl<T: TabledInteger, const N: usize> WriteNumUnchecked<$t> for heapless::Vec<T, N> {
            gen_write_num_unchecked_signed!($t);
        }
    )*)
}

impl_write_num_unchecked_signed! { i8 i16 i32 i64 i128 isize }

impl<T> WriteNumUnchecked<T> for String
where
    Vec<u8>: WriteNumUnchecked<T>,
{
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: T, radix: u8, reversed: bool, from_0: bool) {
        self.as_mut_vec()
            .write_num_unchecked(n, radix, reversed, from_0);
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> WriteNumUnchecked<T> for heapless::String<N>
where
    heapless::Vec<u8, N>: WriteNumUnchecked<T>,
{
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: T, radix: u8, reversed: bool, from_0: bool) {
        self.as_mut_vec()
            .write_num_unchecked(n, radix, reversed, from_0);
    }
}

pub trait NumToString<const N10: usize, const N2: usize, const N16: usize>: Sized {
    fn to_heapless_string(self, reversed: bool, from_0: bool) -> heapless::String<N10>;
    fn to_heapless_string_base2(self, reversed: bool, from_0: bool) -> heapless::String<N2>;
    fn to_heapless_string_base16(self, reversed: bool, from_0: bool) -> heapless::String<N16>;
    fn to_string(self, reversed: bool, from_0: bool) -> String;
    fn to_string_base2(self, reversed: bool, from_0: bool) -> String;
    fn to_string_base16(self, reversed: bool, from_0: bool) -> String;
}

macro_rules! impl_num_to_string {
    ($($t:ty => $n10:literal,)*) => ($(
        impl NumToString<$n10, { Self::BITS as _ }, { (Self::BITS / 4) as _ }> for $t {
            #[inline]
            fn to_heapless_string(self, reversed: bool, from_0: bool) -> heapless::String<$n10> {
                let mut res = heapless::String::new();
                unsafe {
                    res.write_num_unchecked(self, 10, reversed, from_0);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[inline]
            fn to_string(self, reversed: bool, from_0: bool) -> String {
                let mut res = String::with_capacity($n10);
                unsafe {
                    res.write_num_unchecked(self, 10, reversed, from_0);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[inline]
            fn to_heapless_string_base2(
                self,
                reversed: bool,
                from_0: bool,
            ) -> heapless::String<{ Self::BITS as _ }> {
                let mut res = heapless::String::new();
                unsafe {
                    res.write_num_unchecked(self, 2, reversed, from_0);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[inline]
            fn to_string_base2(self, reversed: bool, from_0: bool) -> String {
                let mut res = String::with_capacity(Self::BITS as _);
                unsafe {
                    res.write_num_unchecked(self, 2, reversed, from_0);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[inline]
            fn to_heapless_string_base16(
                self,
                reversed: bool,
                from_0: bool,
            ) -> heapless::String<{ (Self::BITS / 4) as _ }> {
                let mut res = heapless::String::new();
                unsafe {
                    res.write_num_unchecked(self, 16, reversed, from_0);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[inline]
            fn to_string_base16(self, reversed: bool, from_0: bool) -> String {
                let mut res = String::with_capacity((Self::BITS / 4) as _);
                unsafe {
                    res.write_num_unchecked(self, 16, reversed, from_0);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }
        }
    )*)
}

impl_num_to_string! {
    u8 => 3,
    u16 => 5,
    u32 => 10,
    u64 => 20,
    u128 => 39,
    i8 => 4,
    i16 => 6,
    i32 => 11,
    i64 => 20,
    i128 => 40,
}

#[cfg(target_pointer_width = "16")]
impl_num_to_string! { usize => 5, isize => 6, }
#[cfg(target_pointer_width = "32")]
impl_num_to_string! { usize => 10, isize => 11, }
#[cfg(target_pointer_width = "64")]
impl_num_to_string! { usize => 20, isize => 20, }
