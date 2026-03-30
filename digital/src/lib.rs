//! Fast methods for digit manipulation, including conversions from integers
//! to strings or arrays of digits.
//! Based on [rust-lexical](https://github.com/Alexhuszagh/rust-lexical).
#![no_std]

extern crate alloc;

pub use self::{
    digit_count::DigitCount,
    digit_sum::DigitSum,
    format::{Base, Base10, Base16, Base2, DigitFormat, Normal, Radix, Raw, Reversed, ReversedRaw},
    max_len_base10::MaxLenBase10,
    next_2_digits::Next2Digits,
    to_string::IntToString,
    write_digits::WriteIntUnchecked,
};

mod digit;
mod digit_count;
mod digit_sum;
mod format;
mod max_len_base10;
mod next_2_digits;
mod to_string;
mod write_digits;

/// Duplicate exports in `prelude` to comply with `clippy::wildcard_imports`.
pub mod prelude {
    pub use super::*;
}
