pub trait MaxLenBase10 {
    const MAX_LEN_BASE10: usize;
}

impl MaxLenBase10 for u8 {
    const MAX_LEN_BASE10: usize = "255".len();
}

impl MaxLenBase10 for i8 {
    const MAX_LEN_BASE10: usize = "-128".len();
}

impl MaxLenBase10 for u16 {
    const MAX_LEN_BASE10: usize = "65535".len();
}

impl MaxLenBase10 for i16 {
    const MAX_LEN_BASE10: usize = "-32768".len();
}

impl MaxLenBase10 for u32 {
    const MAX_LEN_BASE10: usize = "4294967295".len();
}

impl MaxLenBase10 for i32 {
    const MAX_LEN_BASE10: usize = "-2147483648".len();
}

impl MaxLenBase10 for u64 {
    const MAX_LEN_BASE10: usize = "18446744073709551615".len();
}

impl MaxLenBase10 for i64 {
    const MAX_LEN_BASE10: usize = "-9223372036854775808".len();
}

impl MaxLenBase10 for u128 {
    const MAX_LEN_BASE10: usize = "340282366920938463463374607431768211455".len();
}

impl MaxLenBase10 for i128 {
    const MAX_LEN_BASE10: usize = "-170141183460469231731687303715884105728".len();
}

#[expect(clippy::use_self)]
impl MaxLenBase10 for usize {
    const MAX_LEN_BASE10: usize = {
        #[cfg(target_pointer_width = "16")]
        {
            u16::MAX_LEN_BASE10
        }
        #[cfg(target_pointer_width = "32")]
        {
            u32::MAX_LEN_BASE10
        }
        #[cfg(target_pointer_width = "64")]
        {
            u64::MAX_LEN_BASE10
        }
    };
}

impl MaxLenBase10 for isize {
    const MAX_LEN_BASE10: usize = {
        #[cfg(target_pointer_width = "16")]
        {
            i16::MAX_LEN_BASE10
        }
        #[cfg(target_pointer_width = "32")]
        {
            i32::MAX_LEN_BASE10
        }
        #[cfg(target_pointer_width = "64")]
        {
            i64::MAX_LEN_BASE10
        }
    };
}
