use alloc::{string::String, vec::Vec};

use super::{
    digit::Digit,
    format::{Base, Base10, DigitFormat, Normal, Radix},
    DigitCount,
};

#[inline]
unsafe fn write_digit<D: Digit, F: DigitFormat, R: Radix>(
    buffer: &mut [D],
    index: &mut usize,
    digit: u8,
) {
    if !F::REVERSED {
        *index -= 1;
    }
    *buffer.get_unchecked_mut(*index) = D::from_u8(if F::RAW {
        digit
    } else if R::BASE != Base::Base16 {
        digit + b'0'
    } else {
        *(*b"0123456789abcdef").get_unchecked(digit as usize)
    });
    if F::REVERSED {
        *index = index.wrapping_add(1);
    }
}

#[inline]
unsafe fn write_2_digits<D: Copy, F: DigitFormat>(
    buffer: &mut [D],
    index: &mut usize,
    r: usize,
    table: &[D],
) {
    if F::REVERSED {
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

pub trait WriteIntUnchecked<T> {
    /// # Safety
    ///
    /// `self.len() + n.to_string().len()` must be `<= self.capacity()`
    unsafe fn write_int_unchecked(&mut self, n: T) {
        self.write_int_unchecked_with::<Normal, Base10>(n);
    }

    /// # Safety
    ///
    /// `self.len() + n.to_string().len()` must be `<= self.capacity()`
    unsafe fn write_int_unchecked_with<F: DigitFormat, R: Radix>(&mut self, n: T);
}

macro_rules! gen_write_int_unchecked_unsigned {
    ($t:ty) => {
        #[inline]
        unsafe fn write_int_unchecked_with<F: DigitFormat, R: Radix>(&mut self, n: $t) {
            let len = self.len();
            let index = if F::REVERSED {
                0
            } else {
                match R::BASE {
                    Base::Base2 => n.count_digits_base2(),
                    Base::Base10 => {
                        if <$t>::BITS == 128 && u64::try_from(n).is_ok() {
                            (n as u64).count_digits_base10()
                        } else {
                            n.count_digits_base10()
                        }
                    }
                    Base::Base16 => n.count_digits_base16(),
                }
            };
            let index2 = n.write_digits::<_, F, R>(
                core::slice::from_raw_parts_mut(self.as_mut_ptr().add(len), self.capacity() - len),
                index,
            );
            self.set_len(len + if F::REVERSED { index2 } else { index });
        }
    };
}

trait WriteDigits {
    unsafe fn write_digits<D: Digit, F: DigitFormat, R: Radix>(
        self,
        buffer: &mut [D],
        index: usize,
    ) -> usize;
}

macro_rules! impl_write_digits {
    ($($t:ty)*) => ($(
        impl WriteDigits for $t {
            #[inline]
            unsafe fn write_digits<D: Digit, F: DigitFormat, R: Radix>(
                mut self,
                buffer: &mut [D],
                mut index: usize,
            ) -> usize {
                const {
                    if matches!(R::BASE, Base::Base16) && F::RAW {
                        panic!("raw encoding is not implemented for hex");
                    }
                }
                let table: &[D] = match R::BASE {
                    Base::Base2 => {
                        if F::REVERSED {
                            if F::RAW {
                                D::TO_BASE2_REVERSED_RAW
                            } else {
                                D::TO_BASE2_REVERSED
                            }
                        } else if F::RAW {
                            D::TO_BASE2_RAW
                        } else {
                            D::TO_BASE2
                        }
                    },
                    Base::Base10 => {
                        if F::REVERSED {
                            if F::RAW {
                                D::TO_BASE10_REVERSED_RAW
                            } else {
                                D::TO_BASE10_REVERSED
                            }
                        } else if F::RAW {
                            D::TO_BASE10_RAW
                        } else {
                            D::TO_BASE10
                        }
                    },
                    Base::Base16 => {
                        if F::RAW {
                            unreachable!()
                        } else if F::REVERSED {
                            D::TO_BASE16_REVERSED
                        } else {
                            D::TO_BASE16
                        }
                    },
                };

                let radix = u8::from(R::BASE) as $t;
                if let Some(radix2) = radix.checked_mul(radix) {
                    if let Some(radix4) = radix2.checked_mul(radix2) {
                        while self >= radix4 {
                            let r = self % radix4;
                            self /= radix4;
                            let r1 = (2 * (r / radix2)) as usize;
                            let r2 = (2 * (r % radix2)) as usize;

                            write_2_digits::<_, F>(buffer, &mut index, r2, table);
                            write_2_digits::<_, F>(buffer, &mut index, r1, table);
                        }
                    }

                    while self >= radix2 {
                        let r = (2 * (self % radix2)) as usize;
                        self /= radix2;

                        write_2_digits::<_, F>(buffer, &mut index, r, table);
                    }
                }

                if self < radix {
                    let r = self as _;
                    write_digit::<_, F, R>(buffer, &mut index, r);
                } else {
                    let r = 2 * self as usize;
                    write_2_digits::<_, F>(buffer, &mut index, r, table);
                }

                index
            }
        }
    )*)
}

impl_write_digits! { u8 u16 u32 u64 usize }

macro_rules! impl_write_int_unchecked_unsigned {
    ($($t:ty)*) => ($(
        impl<D: Digit> WriteIntUnchecked<$t> for Vec<D> {
            gen_write_int_unchecked_unsigned!($t);
        }

        #[cfg(feature = "heapless")]
        impl<D: Digit, const N: usize> WriteIntUnchecked<$t> for heapless::Vec<D, N> {
            gen_write_int_unchecked_unsigned!($t);
        }
    )*)
}

impl_write_int_unchecked_unsigned! { u8 u16 u32 u64 u128 usize }

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
const fn u128_divrem<R: Radix>(n: u128) -> (u128, u64) {
    match R::BASE {
        Base::Base2 | Base::Base16 => (n >> 64, n as u64),
        Base::Base10 => fast_u128_divrem(
            n,
            10_000_000_000_000_000_000,
            9_671_406_556_917_033_397_649_408,
            19,
            156_927_543_384_667_019_095_894_735_580_191_660_403,
            62,
        ),
    }
}

#[inline]
unsafe fn write_step_digits<D: Digit, F: DigitFormat, R: Radix>(
    value: u64,
    buffer: &mut [D],
    index: usize,
    step: usize,
) -> usize {
    let start = index;
    let index = value.write_digits::<_, F, R>(buffer, index);
    let (end, zeros) = if F::REVERSED {
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
        if F::RAW { 0 } else { b'0' },
        zeros.len(),
    );
    end
}

impl WriteDigits for u128 {
    #[inline]
    unsafe fn write_digits<D: Digit, F: DigitFormat, R: Radix>(
        self,
        buffer: &mut [D],
        mut index: usize,
    ) -> usize {
        if let Ok(value) = TryInto::<u64>::try_into(self) {
            return value.write_digits::<_, F, R>(buffer, index);
        }

        let step = match R::BASE {
            Base::Base2 => 64,
            Base::Base10 => 19,
            Base::Base16 => 16,
        };
        let (value, low) = u128_divrem::<R>(self);
        index = write_step_digits::<_, F, R>(low, buffer, index, step);
        if let Ok(value) = TryInto::<u64>::try_into(value) {
            return value.write_digits::<_, F, R>(buffer, index);
        }

        let (value, mid) = u128_divrem::<R>(value);
        index = write_step_digits::<_, F, R>(mid, buffer, index, step);
        if index != 0 {
            index = (value as u64).write_digits::<_, F, R>(buffer, index);
        }

        index
    }
}

macro_rules! gen_write_int_unchecked_signed {
    ($t:ty) => {
        #[inline]
        unsafe fn write_int_unchecked_with<F: DigitFormat, R: Radix>(&mut self, n: $t) {
            if n < 0 && !F::REVERSED {
                core::ptr::write(self.as_mut_ptr().add(self.len()), D::from_u8(b'-'));
                self.set_len(self.len() + 1);
            }
            self.write_int_unchecked_with::<F, R>(n.unsigned_abs());
            if n < 0 && F::REVERSED {
                core::ptr::write(self.as_mut_ptr().add(self.len()), D::from_u8(b'-'));
                self.set_len(self.len() + 1);
            }
        }
    };
}

macro_rules! impl_write_int_unchecked_signed {
    ($($t:ty)*) => ($(
        impl<D: Digit> WriteIntUnchecked<$t> for Vec<D> {
            gen_write_int_unchecked_signed!($t);
        }

        #[cfg(feature = "heapless")]
        impl<D: Digit, const N: usize> WriteIntUnchecked<$t> for heapless::Vec<D, N> {
            gen_write_int_unchecked_signed!($t);
        }
    )*)
}

impl_write_int_unchecked_signed! { i8 i16 i32 i64 i128 isize }

impl<T> WriteIntUnchecked<T> for String
where
    Vec<u8>: WriteIntUnchecked<T>,
{
    #[inline]
    unsafe fn write_int_unchecked_with<F: DigitFormat, R: Radix>(&mut self, n: T) {
        self.as_mut_vec().write_int_unchecked_with::<F, R>(n);
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> WriteIntUnchecked<T> for heapless::String<N>
where
    heapless::Vec<u8, N>: WriteIntUnchecked<T>,
{
    #[inline]
    unsafe fn write_int_unchecked_with<F: DigitFormat, R: Radix>(&mut self, n: T) {
        self.as_mut_vec().write_int_unchecked_with::<F, R>(n);
    }
}
