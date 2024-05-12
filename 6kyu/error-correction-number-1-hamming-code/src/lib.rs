//! <https://www.codewars.com/kata/5ef9ca8b76be6d001d5e1c3e/train/rust>

use unchecked_core::CopyFromSliceUnchecked;

pub fn encode(text: &str) -> String {
    assert!(text.is_ascii());

    let len = 24 * text.len();
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };

    let mut res_i = 0;
    for b in text.bytes() {
        unsafe {
            res.get_unchecked_mut(res_i..res_i + 24)
                .copy_from_slice_unchecked(ENCODE_TABLE.get_unchecked(b as usize));
        }
        res_i += 24;
    }

    unsafe { String::from_utf8_unchecked(res) }
}

pub fn decode(bits: &str) -> String {
    let bits = bits.as_bytes();
    let len = bits.len() / 24;
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };

    let mut res_i = 0;
    let mut b_i = 0;
    while res_i < len {
        let mut byte = 0u8;
        for j in (0..8).rev() {
            if unsafe { bits.get_unchecked(b_i..b_i + 3) }
                .iter()
                .sum::<u8>()
                >= b'1' + b'1' + b'0'
            {
                byte += 1 << j;
            }
            b_i += 3;
        }
        assert!(byte.is_ascii());
        res[res_i] = byte;
        res_i += 1;
    }

    unsafe { String::from_utf8_unchecked(res) }
}

const ENCODE_TABLE: [[u8; 24]; 128] = {
    let mut res = [[0; 24]; 128];
    let mut i = 0;
    while i < 128 {
        res[i] = encode_byte(i as _);
        i += 1;
    }
    res
};

const fn encode_byte(byte: u8) -> [u8; 24] {
    let mut res = [b'0'; 24];
    let mut i = 0;
    let mut bit = 0;
    while bit < 8 {
        if byte & (1 << (7 - bit)) != 0 {
            res[i] = b'1';
            res[i + 1] = b'1';
            res[i + 2] = b'1';
        }
        i += 3;
        bit += 1;
    }
    res
}
