//! <https://www.codewars.com/kata/5efae11e2d12df00331f91a6/train/rust>

use lexical_core::{write_with_options_unchecked, NumberFormatBuilder, WriteIntegerOptions};
use my_prelude::prelude::*;

fn my_md5_process_block(word0: u32, word1: u32) -> (u32, u32, u32, u32) {
    const A: u32 = 0x6745_2301;
    const B: u32 = 0xefcd_ab89;
    const C: u32 = 0x98ba_dcfe;
    const D: u32 = 0x1032_5476;

    let mut a = A;
    let mut b = B;
    let mut c = C;
    let mut d = D;

    let abcd = |f: fn(u32, u32, u32) -> u32, a: u32, b, c, d, x, t, s| {
        let temp = a.wrapping_add(f(b, c, d)).wrapping_add(x).wrapping_add(t);
        let temp = (temp << s) | (temp >> (32 - s));
        b.wrapping_add(temp)
    };

    let f = |x: u32, y, z| (x & y) | (!x & z);

    a = abcd(f, a, b, c, d, word0, 0xd76a_a478, 7);
    d = abcd(f, d, a, b, c, word1, 0xe8c7_b756, 12);
    c = abcd(f, c, d, a, b, 0, 0x2420_70db, 17);
    b = abcd(f, b, c, d, a, 0, 0xc1bd_ceee, 22);

    a = abcd(f, a, b, c, d, 0, 0xf57c_0faf, 7);
    d = abcd(f, d, a, b, c, 0, 0x4787_c62a, 12);
    c = abcd(f, c, d, a, b, 0, 0xa830_4613, 17);
    b = abcd(f, b, c, d, a, 0, 0xfd46_9501, 22);

    a = abcd(f, a, b, c, d, 0, 0x6980_98d8, 7);
    d = abcd(f, d, a, b, c, 0, 0x8b44_f7af, 12);
    c = abcd(f, c, d, a, b, 0, 0xffff_5bb1, 17);
    b = abcd(f, b, c, d, a, 0, 0x895c_d7be, 22);

    a = abcd(f, a, b, c, d, 0, 0x6b90_1122, 7);
    d = abcd(f, d, a, b, c, 0, 0xfd98_7193, 12);
    c = abcd(f, c, d, a, b, 40, 0xa679_438e, 17);
    b = abcd(f, b, c, d, a, 0, 0x49b4_0821, 22);

    let g = |x, y, z: u32| (x & z) | (y & !z);

    a = abcd(g, a, b, c, d, word1, 0xf61e_2562, 5);
    d = abcd(g, d, a, b, c, 0, 0xc040_b340, 9);
    c = abcd(g, c, d, a, b, 0, 0x265e_5a51, 14);
    b = abcd(g, b, c, d, a, word0, 0xe9b6_c7aa, 20);

    a = abcd(g, a, b, c, d, 0, 0xd62f_105d, 5);
    d = abcd(g, d, a, b, c, 0, 0x0244_1453, 9);
    c = abcd(g, c, d, a, b, 0, 0xd8a1_e681, 14);
    b = abcd(g, b, c, d, a, 0, 0xe7d3_fbc8, 20);

    a = abcd(g, a, b, c, d, 0, 0x21e1_cde6, 5);
    d = abcd(g, d, a, b, c, 40, 0xc337_07d6, 9);
    c = abcd(g, c, d, a, b, 0, 0xf4d5_0d87, 14);
    b = abcd(g, b, c, d, a, 0, 0x455a_14ed, 20);

    a = abcd(g, a, b, c, d, 0, 0xa9e3_e905, 5);
    d = abcd(g, d, a, b, c, 0, 0xfcef_a3f8, 9);
    c = abcd(g, c, d, a, b, 0, 0x676f_02d9, 14);
    b = abcd(g, b, c, d, a, 0, 0x8d2a_4c8a, 20);

    let h = |x, y, z| x ^ y ^ z;

    a = abcd(h, a, b, c, d, 0, 0xfffa_3942, 4);
    d = abcd(h, d, a, b, c, 0, 0x8771_f681, 11);
    c = abcd(h, c, d, a, b, 0, 0x6d9d_6122, 16);
    b = abcd(h, b, c, d, a, 40, 0xfde5_380c, 23);

    a = abcd(h, a, b, c, d, word1, 0xa4be_ea44, 4);
    d = abcd(h, d, a, b, c, 0, 0x4bde_cfa9, 11);
    c = abcd(h, c, d, a, b, 0, 0xf6bb_4b60, 16);
    b = abcd(h, b, c, d, a, 0, 0xbebf_bc70, 23);

    a = abcd(h, a, b, c, d, 0, 0x289b_7ec6, 4);
    d = abcd(h, d, a, b, c, word0, 0xeaa1_27fa, 11);
    c = abcd(h, c, d, a, b, 0, 0xd4ef_3085, 16);
    b = abcd(h, b, c, d, a, 0, 0x0488_1d05, 23);

    a = abcd(h, a, b, c, d, 0, 0xd9d4_d039, 4);
    d = abcd(h, d, a, b, c, 0, 0xe6db_99e5, 11);
    c = abcd(h, c, d, a, b, 0, 0x1fa2_7cf8, 16);
    b = abcd(h, b, c, d, a, 0, 0xc4ac_5665, 23);

    let i = |x, y, z: u32| y ^ (x | !z);

    a = abcd(i, a, b, c, d, word0, 0xf429_2244, 6);
    d = abcd(i, d, a, b, c, 0, 0x432a_ff97, 10);
    c = abcd(i, c, d, a, b, 40, 0xab94_23a7, 15);
    b = abcd(i, b, c, d, a, 0, 0xfc93_a039, 21);

    a = abcd(i, a, b, c, d, 0, 0x655b_59c3, 6);
    d = abcd(i, d, a, b, c, 0, 0x8f0c_cc92, 10);
    c = abcd(i, c, d, a, b, 0, 0xffef_f47d, 15);
    b = abcd(i, b, c, d, a, word1, 0x8584_5dd1, 21);

    a = abcd(i, a, b, c, d, 0, 0x6fa8_7e4f, 6);
    d = abcd(i, d, a, b, c, 0, 0xfe2c_e6e0, 10);
    c = abcd(i, c, d, a, b, 0, 0xa301_4314, 15);
    b = abcd(i, b, c, d, a, 0, 0x4e08_11a1, 21);

    a = abcd(i, a, b, c, d, 0, 0xf753_7e82, 6);
    d = abcd(i, d, a, b, c, 0, 0xbd3a_f235, 10);
    c = abcd(i, c, d, a, b, 0, 0x2ad7_d2bb, 15);
    b = abcd(i, b, c, d, a, 0, 0xeb86_d391, 21);

    (
        a.wrapping_add(A),
        b.wrapping_add(B),
        c.wrapping_add(C),
        d.wrapping_add(D),
    )
}

fn my_md5(word0: u32, word1: u32) -> String {
    const FORMAT: u128 = NumberFormatBuilder::hexadecimal();

    let mut res = String::with_capacity(32);

    let (a, b, c, d) = my_md5_process_block(word0, word1);

    for num in [a, b, c, d] {
        for b in num.to_le_bytes() {
            unsafe {
                if b < 16 {
                    res.push_unchecked('0');
                }
                let len = res.len();
                let written_len = write_with_options_unchecked::<_, FORMAT>(
                    b,
                    core::slice::from_raw_parts_mut(
                        res.as_mut_ptr().add(len),
                        res.capacity() - len,
                    ),
                    &WriteIntegerOptions::new(),
                )
                .len();
                res.as_mut_vec().set_len(len + written_len);
            }
        }
    }

    res
}

#[allow(clippy::result_unit_err)]
pub fn crack(mut string: String) -> Result<i32, ()> {
    // converting to upper since could not make lexical to write in lowercase
    for b in unsafe { string.as_bytes_mut() } {
        if *b >= b'a' {
            *b -= b'a' - b'A';
        }
    }

    let mut word0 = 0x3030_3030;
    for d0 in 0..10 {
        for d1 in 0..10 {
            for d2 in 0..10 {
                for d3 in 0..10 {
                    let mut word1 = 0x8030;
                    for d4 in 0..10 {
                        if my_md5(word0, word1) == string {
                            return Ok(10000 * d0 + 1000 * d1 + 100 * d2 + 10 * d3 + d4);
                        }
                        word1 += 1;
                    }
                    word0 += 0x0100_0000;
                }
                word0 -= 0x09ff_0000;
            }
            word0 -= 0x09ff_00;
        }
        word0 -= 0x09ff;
    }

    Err(())
}
