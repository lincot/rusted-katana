//! <https://www.codewars.com/kata/597eeb0136f4ae84f9000001/train/rust>

use digital::prelude::*;

pub fn parse_bank_account(bank_account: &str) -> u64 {
    let bank_account = bank_account.as_bytes();
    let len = bank_account.iter().position(|&b| b == b'\n').unwrap();
    assert!(
        bank_account.len() == 3 * (len + 1)
            && len <= u64::MAX_LEN_BASE10 * 3 + 1
            && len.is_multiple_of(3)
    );
    let mut start = 0;
    let mut res = 0;
    while start < len {
        let len = len + 1;
        let digit = unsafe {
            if *bank_account.get_unchecked(start + 1) == b' ' {
                if *bank_account.get_unchecked(start + len) == b' ' {
                    1
                } else {
                    4
                }
            } else if *bank_account.get_unchecked(start + len) == b' ' {
                if *bank_account.get_unchecked(start + len + 1) == b' ' {
                    7
                } else if *bank_account.get_unchecked(start + 2 * len) == b' ' {
                    3
                } else {
                    2
                }
            } else if *bank_account.get_unchecked(start + len + 1) == b' ' {
                0
            } else if *bank_account.get_unchecked(start + len + 2) == b' ' {
                if *bank_account.get_unchecked(start + 2 * len) == b' ' {
                    5
                } else {
                    6
                }
            } else if *bank_account.get_unchecked(start + 2 * len) == b' ' {
                9
            } else {
                8
            }
        };
        res *= 10;
        res += digit;

        start += 3;
    }
    res
}
