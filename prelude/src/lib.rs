#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub trait PushUnchecked<T> {
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

impl PushUnchecked<char> for String {
    /// # Safety
    ///
    /// `self.len() + ch.len_utf8()` must be `<= self.capacity()`
    #[inline]
    unsafe fn push_unchecked(&mut self, ch: char) {
        let len = self.len();
        let ptr = self.as_mut_vec().as_mut_ptr().add(len);
        let count = ch.len_utf8();
        debug_assert!(len + count <= self.capacity());
        match count {
            1 => {
                core::ptr::write(ptr, ch as u8);
            }
            2 => {
                core::ptr::write(ptr, (ch as u32 >> 6 & 0x1F) as u8 | 0b1100_0000);
                core::ptr::write(ptr.add(1), (ch as u32 & 0x3F) as u8 | 0b1000_0000);
            }
            3 => {
                core::ptr::write(ptr, (ch as u32 >> 12 & 0x0F) as u8 | 0b1110_0000);
                core::ptr::write(ptr.add(1), (ch as u32 >> 6 & 0x3F) as u8 | 0b1000_0000);
                core::ptr::write(ptr.add(2), (ch as u32 & 0x3F) as u8 | 0b1000_0000);
            }
            4 => {
                core::ptr::write(ptr, (ch as u32 >> 18 & 0x07) as u8 | 0b1111_0000);
                core::ptr::write(ptr.add(1), (ch as u32 >> 12 & 0x3F) as u8 | 0b1000_0000);
                core::ptr::write(ptr.add(2), (ch as u32 >> 6 & 0x3F) as u8 | 0b1000_0000);
                core::ptr::write(ptr.add(3), (ch as u32 & 0x3F) as u8 | 0b1000_0000);
            }
            _ => core::hint::unreachable_unchecked(),
        }
        self.as_mut_vec().set_len(len + count);
    }
}

pub trait ExtendUnchecked<T, I> {
    /// # Safety
    ///
    /// `self.len() + iter.count()` must be `<= self.capacity()`
    unsafe fn extend_unchecked(&mut self, iter: I);
}

impl<I: IntoIterator<Item = char>> ExtendUnchecked<char, I> for String {
    #[inline]
    unsafe fn extend_unchecked(&mut self, iter: I) {
        for value in iter {
            self.push_unchecked(value);
        }
    }
}

pub trait ExtendFromSliceUnchecked<T> {
    /// # Safety
    ///
    /// `other.len()` must be `<= self.capacity() - self.len()`
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]);
}

impl<T: Clone> ExtendFromSliceUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let len = self.len();
        let count = other.len();
        debug_assert!(count <= self.capacity() - len);
        if count > self.capacity() - len {
            core::hint::unreachable_unchecked();
        }
        self.extend_from_slice(other);
    }
}

#[cfg(feature = "heapless")]
impl<T: Copy, const N: usize> ExtendFromSliceUnchecked<T> for heapless::Vec<T, N> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let len = self.len();
        let count = other.len();
        debug_assert!(count <= self.capacity() - len);
        core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
        self.set_len(len + count);
    }
}

pub trait ExtendFromWithinUnchecked {
    /// # Safety
    ///
    /// - `src` needs to be valid index
    /// - `self.capacity() - self.len()` must be `>= src.len()`
    unsafe fn extend_from_within_unchecked(&mut self, src: core::ops::Range<usize>);
}

impl<T: Copy> ExtendFromWithinUnchecked for Vec<T> {
    unsafe fn extend_from_within_unchecked(&mut self, src: core::ops::Range<usize>) {
        let count = src.len();
        debug_assert!(src.start <= src.end || src.end <= self.len());
        debug_assert!(self.capacity() - self.len() >= count);
        let source = self.get_unchecked(src);
        core::ptr::copy_nonoverlapping(source.as_ptr(), self.as_mut_ptr().add(self.len()), count);
        self.set_len(self.len() + count);
    }
}

pub trait PushStrUnchecked {
    /// # Safety
    ///
    /// `self.len() + string.len()` must be `<= self.capacity()`
    unsafe fn push_str_unchecked(&mut self, string: &str);
}

impl PushStrUnchecked for String {
    #[inline]
    unsafe fn push_str_unchecked(&mut self, string: &str) {
        self.as_mut_vec()
            .extend_from_slice_unchecked(string.as_bytes());
    }
}

pub const USIZE_MAX_LEN: usize = {
    let mut n = usize::MAX;
    let mut res = 0;
    while n != 0 {
        res += 1;
        n /= 10;
    }
    res
};

// all the code below is based on https://github.com/Alexhuszagh/rust-lexical

const DIGIT_TO_BASE10_SQUARED_FROM_0: [u8; 200] = [
    0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5,
    1, 6, 1, 7, 1, 8, 1, 9, 2, 0, 2, 1, 2, 2, 2, 3, 2, 4, 2, 5, 2, 6, 2, 7, 2, 8, 2, 9, 3, 0, 3, 1,
    3, 2, 3, 3, 3, 4, 3, 5, 3, 6, 3, 7, 3, 8, 3, 9, 4, 0, 4, 1, 4, 2, 4, 3, 4, 4, 4, 5, 4, 6, 4, 7,
    4, 8, 4, 9, 5, 0, 5, 1, 5, 2, 5, 3, 5, 4, 5, 5, 5, 6, 5, 7, 5, 8, 5, 9, 6, 0, 6, 1, 6, 2, 6, 3,
    6, 4, 6, 5, 6, 6, 6, 7, 6, 8, 6, 9, 7, 0, 7, 1, 7, 2, 7, 3, 7, 4, 7, 5, 7, 6, 7, 7, 7, 8, 7, 9,
    8, 0, 8, 1, 8, 2, 8, 3, 8, 4, 8, 5, 8, 6, 8, 7, 8, 8, 8, 9, 9, 0, 9, 1, 9, 2, 9, 3, 9, 4, 9, 5,
    9, 6, 9, 7, 9, 8, 9, 9,
];

const DIGIT_TO_BASE10_SQUARED: [u8; 200] = {
    let mut arr = [0; 200];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = DIGIT_TO_BASE10_SQUARED_FROM_0[i] + b'0';
        i += 1;
    }
    arr
};

#[inline]
unsafe fn write_digit<T: From<u8>>(
    buffer: &mut [T],
    index: &mut usize,
    r: u8,
    reversed: bool,
    from_0: bool,
) {
    if reversed {
        *index = index.wrapping_add(1);
    } else {
        *index -= 1;
    }
    *buffer.get_unchecked_mut(*index) = (if from_0 { r } else { r + b'0' }).into();
}

#[inline]
unsafe fn write_2_digits<T: From<u8>>(
    buffer: &mut [T],
    index: &mut usize,
    r: usize,
    reversed: bool,
    from_0: bool,
) {
    let table = if from_0 {
        DIGIT_TO_BASE10_SQUARED_FROM_0
    } else {
        DIGIT_TO_BASE10_SQUARED
    };
    if reversed {
        *index = index.wrapping_add(1);
    } else {
        *index -= 1;
    }
    *buffer.get_unchecked_mut(*index) = (*table.get_unchecked(r + 1)).into();
    if reversed {
        *index = index.wrapping_add(1);
    } else {
        *index -= 1;
    }
    *buffer.get_unchecked_mut(*index) = (*table.get_unchecked(r)).into();
}

trait WriteDigits {
    unsafe fn write_digits<T: From<u8>>(
        self,
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

pub trait CountDigits {
    fn count_digits(self) -> usize;
}

pub trait WriteNumUnchecked<T> {
    /// # Safety
    ///
    /// `self.len() + n.to_string().len()` must be `<= self.capacity()`
    unsafe fn write_num_unchecked(&mut self, n: T, reversed: bool, from_0: bool);
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
                let shift = unsafe { TABLE[..Self::BITS as _].get_unchecked(self.fast_log2()) };
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

impl CountDigits for u8 {
    fn count_digits(self) -> usize {
        self.fast_digit_count()
    }
}

impl CountDigits for u16 {
    fn count_digits(self) -> usize {
        self.fast_digit_count()
    }
}

impl CountDigits for u32 {
    fn count_digits(self) -> usize {
        self.fast_digit_count()
    }
}

impl CountDigits for u64 {
    fn count_digits(self) -> usize {
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

impl CountDigits for usize {
    fn count_digits(self) -> usize {
        if Self::BITS == 16 {
            (self as u16).count_digits()
        } else if Self::BITS == 32 {
            (self as u32).count_digits()
        } else if Self::BITS == 64 {
            (self as u64).count_digits()
        } else {
            unimplemented!()
        }
    }
}

impl CountDigits for u128 {
    fn count_digits(self) -> usize {
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
        unsafe fn write_num_unchecked(&mut self, n: $t, reversed: bool, from_0: bool) {
            let len = self.len();
            let index = if reversed {
                usize::MAX
            } else if <$t>::BITS == 128 && u64::try_from(n).is_ok() {
                (n as u64).count_digits()
            } else {
                n.count_digits()
            };
            let index2 = n.write_digits(
                core::slice::from_raw_parts_mut(self.as_mut_ptr().add(len), self.capacity() - len),
                index,
                reversed,
                from_0,
            );
            self.set_len(len + if reversed { index2 + 1 } else { index });
        }
    };
}

macro_rules! impl_write_num_unchecked_unsigned {
    ($($t:ty)*) => ($(
        impl WriteDigits for $t {
            #[inline]
            unsafe fn write_digits<T: From<u8>>(
                mut self,
                buffer: &mut [T],
                mut index: usize,
                reversed: bool,
                from_0: bool,
            ) -> usize {
                let radix = 10;
                let radix2 = radix * radix;

                if Self::BITS >= 16 {
                    let radix4 = radix2 * radix2;

                    while self >= radix4 {
                        let r = self % radix4;
                        self /= radix4;
                        let r1 = (2 * (r / radix2)) as usize;
                        let r2 = (2 * (r % radix2)) as usize;

                        write_2_digits(buffer, &mut index, r2, reversed, from_0);
                        write_2_digits(buffer, &mut index, r1, reversed, from_0);
                    }
                }

                while self >= radix2 {
                    let r = (2 * (self % radix2)) as usize;
                    self /= radix2;

                    write_2_digits(buffer, &mut index, r, reversed, from_0);
                }

                if self < radix {
                    let r = self as _;
                    write_digit(buffer, &mut index, r, reversed, from_0);
                } else {
                    let r = (2 * self) as usize;
                    write_2_digits(buffer, &mut index, r, reversed, from_0);
                }

                index
            }
        }

        impl<T: From<u8>> WriteNumUnchecked<$t> for Vec<T> {
            gen_write_num_unchecked_unsigned!($t);
        }

        #[cfg(feature = "heapless")]
        impl<T: From<u8>, const N: usize> WriteNumUnchecked<$t> for heapless::Vec<T, N> {
            gen_write_num_unchecked_unsigned!($t);
        }
    )*)
}

impl_write_num_unchecked_unsigned! { u8 u32 u64 usize }

#[inline]
const fn mulhi_u128_u64(x: u128, y: u128) -> u128 {
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
const fn fast_u128_u64_divrem(
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
        mulhi_u128_u64(n, factor) >> factor_shr
    };
    let rem = (n - quot * d as u128) as u64;
    (quot, rem)
}

#[inline]
const fn divrem_u128_u64_10(n: u128) -> (u128, u64) {
    fast_u128_u64_divrem(
        n,
        10_000_000_000_000_000_000,
        9_671_406_556_917_033_397_649_408,
        19,
        156_927_543_384_667_019_095_894_735_580_191_660_403,
        62,
    )
}

#[inline]
unsafe fn write_step_digits<T: From<u8>>(
    value: u64,
    buffer: &mut [T],
    index: usize,
    step: usize,
    reversed: bool,
    from_0: bool,
) -> usize {
    let start = index;
    let index = value.write_digits(buffer, index, reversed, from_0);
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
    unsafe fn write_digits<T: From<u8>>(
        self,
        buffer: &mut [T],
        mut index: usize,
        reversed: bool,
        from_0: bool,
    ) -> usize {
        if self <= u64::MAX as _ {
            return (self as u64).write_digits(buffer, index, reversed, from_0);
        }

        let step = 19;
        let (value, low) = divrem_u128_u64_10(self);
        index = write_step_digits(low, buffer, index, step, reversed, from_0);
        if value <= u64::MAX as _ {
            return (value as u64).write_digits(buffer, index, reversed, from_0);
        }

        let (_, mid) = divrem_u128_u64_10(value);
        index = write_step_digits(mid, buffer, index, step, reversed, from_0);
        if index != 0 {
            index = (value as u64).write_digits(buffer, index, reversed, from_0);
        }

        index
    }
}

impl<T: From<u8>> WriteNumUnchecked<u128> for Vec<T> {
    gen_write_num_unchecked_unsigned!(u128);
}

#[cfg(feature = "heapless")]
impl<T: From<u8>, const N: usize> WriteNumUnchecked<u128> for heapless::Vec<T, N> {
    gen_write_num_unchecked_unsigned!(u128);
}

macro_rules! gen_write_num_unchecked_signed {
    ($t:ty) => {
        #[inline]
        unsafe fn write_num_unchecked(&mut self, n: $t, reversed: bool, from_0: bool) {
            if n < 0 {
                self.push_unchecked(b'-'.into());
            }
            self.write_num_unchecked(n.unsigned_abs(), reversed, from_0);
        }
    };
}

macro_rules! impl_write_num_unchecked_signed {
    ($($t:ty)*) => ($(
        impl<T: From<u8>> WriteNumUnchecked<$t> for Vec<T> {
            gen_write_num_unchecked_signed!($t);
        }

        #[cfg(feature = "heapless")]
        impl<T: From<u8>, const N: usize> WriteNumUnchecked<$t> for heapless::Vec<T, N> {
            gen_write_num_unchecked_signed!($t);
        }
    )*)
}

impl_write_num_unchecked_signed! { i8 i32 i64 isize }

impl<T> WriteNumUnchecked<T> for String
where
    Vec<u8>: WriteNumUnchecked<T>,
{
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: T, reversed: bool, from_0: bool) {
        self.as_mut_vec().write_num_unchecked(n, reversed, from_0);
    }
}
