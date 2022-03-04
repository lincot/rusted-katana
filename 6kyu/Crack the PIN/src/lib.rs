//! <https://www.codewars.com/kata/5efae11e2d12df00331f91a6/train/rust>

fn my_md5_process_block(word0: u32, word1: u32) -> (u32, u32, u32, u32) {
    const A: u32 = 0x67452301;
    const B: u32 = 0xefcdab89;
    const C: u32 = 0x98badcfe;
    const D: u32 = 0x10325476;

    let mut a = A;
    let mut b = B;
    let mut c = C;
    let mut d = D;

    #[allow(clippy::too_many_arguments)]
    fn abcd(
        f: fn(u32, u32, u32) -> u32,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        x: u32,
        t: u32,
        s: u32,
    ) -> u32 {
        let temp = a.wrapping_add(f(b, c, d)).wrapping_add(x).wrapping_add(t);
        let temp = (temp << s) | (temp >> (32 - s));
        b.wrapping_add(temp)
    }

    const fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    a = abcd(f, a, b, c, d, word0, 0xd76aa478, 7);
    d = abcd(f, d, a, b, c, word1, 0xe8c7b756, 12);
    c = abcd(f, c, d, a, b, 0, 0x242070db, 17);
    b = abcd(f, b, c, d, a, 0, 0xc1bdceee, 22);

    a = abcd(f, a, b, c, d, 0, 0xf57c0faf, 7);
    d = abcd(f, d, a, b, c, 0, 0x4787c62a, 12);
    c = abcd(f, c, d, a, b, 0, 0xa8304613, 17);
    b = abcd(f, b, c, d, a, 0, 0xfd469501, 22);

    a = abcd(f, a, b, c, d, 0, 0x698098d8, 7);
    d = abcd(f, d, a, b, c, 0, 0x8b44f7af, 12);
    c = abcd(f, c, d, a, b, 0, 0xffff5bb1, 17);
    b = abcd(f, b, c, d, a, 0, 0x895cd7be, 22);

    a = abcd(f, a, b, c, d, 0, 0x6b901122, 7);
    d = abcd(f, d, a, b, c, 0, 0xfd987193, 12);
    c = abcd(f, c, d, a, b, 40, 0xa679438e, 17);
    b = abcd(f, b, c, d, a, 0, 0x49b40821, 22);

    const fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    a = abcd(g, a, b, c, d, word1, 0xf61e2562, 5);
    d = abcd(g, d, a, b, c, 0, 0xc040b340, 9);
    c = abcd(g, c, d, a, b, 0, 0x265e5a51, 14);
    b = abcd(g, b, c, d, a, word0, 0xe9b6c7aa, 20);

    a = abcd(g, a, b, c, d, 0, 0xd62f105d, 5);
    d = abcd(g, d, a, b, c, 0, 0x02441453, 9);
    c = abcd(g, c, d, a, b, 0, 0xd8a1e681, 14);
    b = abcd(g, b, c, d, a, 0, 0xe7d3fbc8, 20);

    a = abcd(g, a, b, c, d, 0, 0x21e1cde6, 5);
    d = abcd(g, d, a, b, c, 40, 0xc33707d6, 9);
    c = abcd(g, c, d, a, b, 0, 0xf4d50d87, 14);
    b = abcd(g, b, c, d, a, 0, 0x455a14ed, 20);

    a = abcd(g, a, b, c, d, 0, 0xa9e3e905, 5);
    d = abcd(g, d, a, b, c, 0, 0xfcefa3f8, 9);
    c = abcd(g, c, d, a, b, 0, 0x676f02d9, 14);
    b = abcd(g, b, c, d, a, 0, 0x8d2a4c8a, 20);

    const fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    a = abcd(h, a, b, c, d, 0, 0xfffa3942, 4);
    d = abcd(h, d, a, b, c, 0, 0x8771f681, 11);
    c = abcd(h, c, d, a, b, 0, 0x6d9d6122, 16);
    b = abcd(h, b, c, d, a, 40, 0xfde5380c, 23);

    a = abcd(h, a, b, c, d, word1, 0xa4beea44, 4);
    d = abcd(h, d, a, b, c, 0, 0x4bdecfa9, 11);
    c = abcd(h, c, d, a, b, 0, 0xf6bb4b60, 16);
    b = abcd(h, b, c, d, a, 0, 0xbebfbc70, 23);

    a = abcd(h, a, b, c, d, 0, 0x289b7ec6, 4);
    d = abcd(h, d, a, b, c, word0, 0xeaa127fa, 11);
    c = abcd(h, c, d, a, b, 0, 0xd4ef3085, 16);
    b = abcd(h, b, c, d, a, 0, 0x04881d05, 23);

    a = abcd(h, a, b, c, d, 0, 0xd9d4d039, 4);
    d = abcd(h, d, a, b, c, 0, 0xe6db99e5, 11);
    c = abcd(h, c, d, a, b, 0, 0x1fa27cf8, 16);
    b = abcd(h, b, c, d, a, 0, 0xc4ac5665, 23);

    const fn i(x: u32, y: u32, z: u32) -> u32 {
        y ^ (x | !z)
    }

    a = abcd(i, a, b, c, d, word0, 0xf4292244, 6);
    d = abcd(i, d, a, b, c, 0, 0x432aff97, 10);
    c = abcd(i, c, d, a, b, 40, 0xab9423a7, 15);
    b = abcd(i, b, c, d, a, 0, 0xfc93a039, 21);

    a = abcd(i, a, b, c, d, 0, 0x655b59c3, 6);
    d = abcd(i, d, a, b, c, 0, 0x8f0ccc92, 10);
    c = abcd(i, c, d, a, b, 0, 0xffeff47d, 15);
    b = abcd(i, b, c, d, a, word1, 0x85845dd1, 21);

    a = abcd(i, a, b, c, d, 0, 0x6fa87e4f, 6);
    d = abcd(i, d, a, b, c, 0, 0xfe2ce6e0, 10);
    c = abcd(i, c, d, a, b, 0, 0xa3014314, 15);
    b = abcd(i, b, c, d, a, 0, 0x4e0811a1, 21);

    a = abcd(i, a, b, c, d, 0, 0xf7537e82, 6);
    d = abcd(i, d, a, b, c, 0, 0xbd3af235, 10);
    c = abcd(i, c, d, a, b, 0, 0x2ad7d2bb, 15);
    b = abcd(i, b, c, d, a, 0, 0xeb86d391, 21);

    (
        a.wrapping_add(A),
        b.wrapping_add(B),
        c.wrapping_add(C),
        d.wrapping_add(D),
    )
}

fn my_md5(word0: u32, word1: u32) -> String {
    let (a, b, c, d) = my_md5_process_block(word0, word1);

    let bytes0: [u8; 4] = a.to_le_bytes();
    let bytes1: [u8; 4] = b.to_le_bytes();
    let bytes2: [u8; 4] = c.to_le_bytes();
    let bytes3: [u8; 4] = d.to_le_bytes();

    format!(
        "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes0[0],
        bytes0[1],
        bytes0[2],
        bytes0[3],
        bytes1[0],
        bytes1[1],
        bytes1[2],
        bytes1[3],
        bytes2[0],
        bytes2[1],
        bytes2[2],
        bytes2[3],
        bytes3[0],
        bytes3[1],
        bytes3[2],
        bytes3[3]
    )
}

#[allow(clippy::result_unit_err)]
pub fn crack(string: String) -> Result<i32, ()> {
    let mut word0 = 0x30303030;
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
                    word0 += 0x1000000;
                }
                word0 -= 0x9ff0000;
            }
            word0 -= 0x9ff00;
        }
        word0 -= 0x9ff;
    }

    Err(())
}
