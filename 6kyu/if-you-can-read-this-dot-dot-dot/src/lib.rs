//! <https://www.codewars.com/kata/586538146b56991861000293/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn to_nato(words: &str) -> String {
    let mut res = Vec::with_capacity("November ".len() * words.len());
    unsafe {
        for &b in words.as_bytes() {
            if b == b' ' {
                continue;
            }
            let b = b.to_ascii_lowercase();
            if b.is_ascii_lowercase() {
                res.extend_from_slice_unchecked(
                    [
                        b"Alfa " as &[u8],
                        b"Bravo ",
                        b"Charlie ",
                        b"Delta ",
                        b"Echo ",
                        b"Foxtrot ",
                        b"Golf ",
                        b"Hotel ",
                        b"India ",
                        b"Juliett ",
                        b"Kilo ",
                        b"Lima ",
                        b"Mike ",
                        b"November ",
                        b"Oscar ",
                        b"Papa ",
                        b"Quebec ",
                        b"Romeo ",
                        b"Sierra ",
                        b"Tango ",
                        b"Uniform ",
                        b"Victor ",
                        b"Whiskey ",
                        b"Xray ",
                        b"Yankee ",
                        b"Zulu ",
                    ][(b - b'a') as usize],
                );
            } else {
                res.push_unchecked(b);
                res.push_unchecked(b' ');
            }
        }
        res.pop();
        String::from_utf8_unchecked(res)
    }
}
