use alloc::string::String;

use super::{
    format::{Base10, Base16, Base2, DigitFormat, Normal},
    MaxLenBase10, WriteIntUnchecked,
};

pub trait IntToString<const CAP10: usize, const CAP2: usize, const CAP16: usize>: Sized {
    fn to_string(self) -> String {
        self.to_string_with::<Normal>()
    }
    fn to_string_with<F: DigitFormat>(self) -> String;

    fn to_string_base2(self) -> String {
        self.to_string_base2_with::<Normal>()
    }
    fn to_string_base2_with<F: DigitFormat>(self) -> String;

    fn to_string_base16(self) -> String {
        self.to_string_base16_with::<Normal>()
    }
    fn to_string_base16_with<F: DigitFormat>(self) -> String;

    #[cfg(feature = "heapless")]
    fn to_heapless_string(self) -> heapless::String<CAP10> {
        self.to_heapless_string_with::<Normal>()
    }
    #[cfg(feature = "heapless")]
    fn to_heapless_string_with<F: DigitFormat>(self) -> heapless::String<CAP10>;

    #[cfg(feature = "heapless")]
    fn to_heapless_string_base2(self) -> heapless::String<CAP2> {
        self.to_heapless_string_base2_with::<Normal>()
    }
    #[cfg(feature = "heapless")]
    fn to_heapless_string_base2_with<F: DigitFormat>(self) -> heapless::String<CAP2>;

    #[cfg(feature = "heapless")]
    fn to_heapless_string_base16(self) -> heapless::String<CAP16> {
        self.to_heapless_string_base16_with::<Normal>()
    }
    #[cfg(feature = "heapless")]
    fn to_heapless_string_base16_with<F: DigitFormat>(self) -> heapless::String<CAP16>;
}

macro_rules! impl_int_to_string {
    ($($t:ty)*) => ($(
        #[allow(unused_comparisons)]
        impl
            IntToString<
                { Self::MAX_LEN_BASE10 },
                { (Self::BITS + (Self::MIN < 0) as u32) as _ },
                { (Self::BITS / 4 + (Self::MIN < 0) as u32) as _ },
            > for $t
        {
            #[inline]
            fn to_string_with<F: DigitFormat>(self) -> String {
                let mut res = String::with_capacity(Self::MAX_LEN_BASE10);
                unsafe {
                    res.write_int_unchecked_with::<F, Base10>(self);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[cfg(feature = "heapless")]
            #[inline]
            fn to_heapless_string_with<F: DigitFormat>(
                self,
            ) -> heapless::String<{ Self::MAX_LEN_BASE10 }> {
                let mut res = heapless::String::new();
                unsafe {
                    res.write_int_unchecked_with::<F, Base10>(self);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }


            #[inline]
            fn to_string_base2_with<F: DigitFormat>(self) -> String {
                #[allow(unused_comparisons)]
                let mut res = String::with_capacity((Self::BITS + (Self::MIN < 0) as u32) as _);
                unsafe {
                    res.write_int_unchecked_with::<F, Base2>(self);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[cfg(feature = "heapless")]
            #[inline]
            fn to_heapless_string_base2_with<F: DigitFormat>(
                self,
            ) -> heapless::String<{ (Self::BITS + (Self::MIN < 0) as u32) as _ }> {
                let mut res = heapless::String::new();
                unsafe {
                    res.write_int_unchecked_with::<F, Base2>(self);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[inline]
            fn to_string_base16_with<F: DigitFormat>(self) -> String {
                #[allow(unused_comparisons)]
                let mut res = String::with_capacity((Self::BITS / 4 + (Self::MIN < 0) as u32) as _);
                unsafe {
                    res.write_int_unchecked_with::<F, Base16>(self);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }

            #[cfg(feature = "heapless")]
            #[inline]
            fn to_heapless_string_base16_with<F: DigitFormat>(
                self,
            ) -> heapless::String<{ (Self::BITS / 4 + (Self::MIN < 0) as u32) as _ }> {
                let mut res = heapless::String::new();
                unsafe {
                    res.write_int_unchecked_with::<F, Base16>(self);
                    if res.is_empty() {
                        core::hint::unreachable_unchecked();
                    }
                }
                res
            }
        }
    )*)
}

impl_int_to_string! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
}
