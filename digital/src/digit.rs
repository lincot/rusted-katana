pub trait FromU8 {
    fn from_u8(x: u8) -> Self;
}

macro_rules! impl_from_u8 {
    ($($t:ty)*) => ($(
        impl FromU8 for $t {
            #[inline]
            fn from_u8(x: u8) -> Self {
                x as _
            }
        }
    )*)
}

impl_from_u8! {
    u8 u16 u32 u64 usize
    i8 i16 i32 i64 isize
}

pub trait Digit: Sized + Copy + FromU8 + 'static {
    const TO_BASE2: &'static [Self; 8];
    const TO_BASE2_RAW: &'static [Self; 8];
    const TO_BASE2_REVERSED_RAW: &'static [Self; 8];
    const TO_BASE2_REVERSED: &'static [Self; 8];

    const TO_BASE10: &'static [Self; 200];
    const TO_BASE10_RAW: &'static [Self; 200];
    const TO_BASE10_REVERSED_RAW: &'static [Self; 200];
    const TO_BASE10_REVERSED: &'static [Self; 200];

    const TO_BASE16: &'static [Self; 512];
    const TO_BASE16_REVERSED: &'static [Self; 512];
}

macro_rules! impl_digit {
    ($($t:ty)*) => ($(
        impl Digit for $t {
            const TO_BASE10_RAW: &'static [Self; 200] = &{
                let mut res = [0; 200];
                let mut i = 0;
                while i < 100 {
                    res[i * 2] = (i / 10) as Self;
                    res[i * 2 + 1] = (i % 10) as Self;
                    i += 1;
                }
                res
            };

            const TO_BASE10: &'static [Self; 200] = &{
                let mut res = [0; 200];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE10_RAW[i] + b'0' as Self;
                    i += 1;
                }
                res
            };

            const TO_BASE10_REVERSED_RAW: &'static [Self; 200] = &{
                let mut res = [0; 200];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE10_RAW[i + 1];
                    res[i + 1] = Self::TO_BASE10_RAW[i];
                    i += 2;
                }
                res
            };

            const TO_BASE10_REVERSED: &'static [Self; 200] = &{
                let mut res = [0; 200];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE10_REVERSED_RAW[i] + b'0' as Self;
                    i += 1;
                }
                res
            };

            const TO_BASE2_RAW: &'static [Self; 8] = &[0, 0, 0, 1, 1, 0, 1, 1];

            const TO_BASE2: &'static [Self; 8] = &{
                let mut res = [0; 8];
                let mut i = 0;
                while i < res.len() {
                    res[i] += Self::TO_BASE2_RAW[i] + b'0' as Self;
                    i += 1;
                }
                res
            };

            const TO_BASE2_REVERSED_RAW: &'static [Self; 8] = &{
                let mut res = [0; 8];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE2_RAW[i + 1];
                    res[i + 1] = Self::TO_BASE2_RAW[i];
                    i += 2;
                }
                res
            };

            const TO_BASE2_REVERSED: &'static [Self; 8] = &{
                let mut res = [0; 8];
                let mut i = 0;
                while i < res.len() {
                    res[i] = Self::TO_BASE2_REVERSED_RAW[i] + b'0' as Self;
                    i += 1;
                }
                res
            };

            const TO_BASE16: &'static [Self; 512] = &{
                const HEX: &[u8] = b"0123456789abcdef";
                let mut res = [0; 512];
                let mut i = 0;
                while i < 256 {
                    res[i * 2] = HEX[i >> 4] as Self;
                    res[i * 2 + 1] = HEX[i & 0xf] as Self;
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

impl_digit! {
    u8 u16 u32 u64 usize
    i8 i16 i32 i64 isize
}
