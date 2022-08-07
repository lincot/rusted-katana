//! <https://www.codewars.com/kata/5d98b6b38b0f6c001a461198/train/rust>

use my_prelude::prelude::*;

const B0: &[u8] = b"10";
const B1: &[u8] = b"11";
const B2: &[u8] = b"0110";
const B3: &[u8] = b"0111";
const B4: &[u8] = b"001100";
const B5: &[u8] = b"001101";
const B6: &[u8] = b"001110";
const B7: &[u8] = b"001111";
const B8: &[u8] = b"00011000";
const B9: &[u8] = b"00011001";

pub fn code(s: &str) -> String {
    let mut res = Vec::with_capacity(8 * s.len());

    for b in s.bytes() {
        unsafe {
            match b {
                b'0' => res.extend_from_slice_unchecked(B0),
                b'1' => res.extend_from_slice_unchecked(B1),
                b'2' => res.extend_from_slice_unchecked(B2),
                b'3' => res.extend_from_slice_unchecked(B3),
                b'4' => res.extend_from_slice_unchecked(B4),
                b'5' => res.extend_from_slice_unchecked(B5),
                b'6' => res.extend_from_slice_unchecked(B6),
                b'7' => res.extend_from_slice_unchecked(B7),
                b'8' => res.extend_from_slice_unchecked(B8),
                _ => res.extend_from_slice_unchecked(B9),
            }
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}

pub fn decode(s: &str) -> String {
    let s = s.as_bytes();
    let mut res = Vec::with_capacity(s.len() / 2);

    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with(B0) {
            unsafe { res.push_unchecked(b'0') };
            i += B0.len();
        } else if s[i..].starts_with(B1) {
            unsafe { res.push_unchecked(b'1') };
            i += B1.len();
        } else if s[i..].starts_with(B2) {
            unsafe { res.push_unchecked(b'2') };
            i += B2.len();
        } else if s[i..].starts_with(B3) {
            unsafe { res.push_unchecked(b'3') };
            i += B3.len();
        } else if s[i..].starts_with(B4) {
            unsafe { res.push_unchecked(b'4') };
            i += B4.len();
        } else if s[i..].starts_with(B5) {
            unsafe { res.push_unchecked(b'5') };
            i += B5.len();
        } else if s[i..].starts_with(B6) {
            unsafe { res.push_unchecked(b'6') };
            i += B6.len();
        } else if s[i..].starts_with(B7) {
            unsafe { res.push_unchecked(b'7') };
            i += B7.len();
        } else if s[i..].starts_with(B8) {
            unsafe { res.push_unchecked(b'8') };
            i += B8.len();
        } else if s[i..].starts_with(B9) {
            unsafe { res.push_unchecked(b'9') };
            i += B9.len();
        } else {
            panic!();
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
