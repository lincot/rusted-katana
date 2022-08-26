//! <https://www.codewars.com/kata/583ade15666df5a64e000058/train/rust>

use lexical::{to_string_with_options, NumberFormatBuilder, WriteIntegerOptions};

pub fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 {
        const FORMAT: u128 = NumberFormatBuilder::binary();
        to_string_with_options::<_, FORMAT>(n, &WriteIntegerOptions::new())
    } else {
        // TODO: format with lowercase numbers
        const FORMAT: u128 = NumberFormatBuilder::hexadecimal();
        let mut res = to_string_with_options::<_, FORMAT>(n, &WriteIntegerOptions::new());
        for b in unsafe { res.as_mut_vec() } {
            if (b'A'..=b'Z').contains(b) {
                *b += b'a' - b'A';
            }
        }
        res
    }
}
