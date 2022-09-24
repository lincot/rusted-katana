//! <https://www.codewars.com/kata/5ecc1d68c6029000017d8aaf/train/rust>

#![no_std]

pub fn max_hexagon_beam(n: u8, seq: &[i32]) -> i32 {
    if seq.is_empty() || n == 0 {
        return 0;
    }

    let mut res = i32::MIN;

    let ith = |i| seq[i % seq.len()];

    let n = n as usize;

    // left to right
    let mut beginning = 0;
    for len in n..2 * n {
        let mut cur_sum = 0;

        for i in beginning..beginning + len {
            cur_sum += ith(i);
        }

        res = res.max(cur_sum);
        beginning += len;
    }
    for len in (n..2 * n - 1).rev() {
        let mut cur_sum = 0;

        for i in beginning..beginning + len {
            cur_sum += ith(i);
        }

        res = res.max(cur_sum);
        beginning += len;
    }

    // down (right to left)
    beginning = 0;
    for len in n..2 * n {
        let mut cur_sum = 0;

        let mut hor_len = n;
        let mut i = beginning;
        for _ in 0..n {
            cur_sum += ith(i);
            i += hor_len;
            hor_len += 1;
        }
        i -= 1;
        hor_len -= 3;
        for _ in n..len {
            cur_sum += ith(i);
            i += hor_len;
            hor_len -= 1;
        }

        res = res.max(cur_sum);
        beginning += 1;
    }
    beginning = 2 * n;
    for (len, hor_len0) in (n..2 * n - 1).rev().zip(n..) {
        let mut cur_sum = 0;

        let mut hor_len = hor_len0;
        let mut i = beginning;
        for _ in 0..len - n {
            cur_sum += ith(i);
            hor_len += 1;
            i += hor_len;
        }
        hor_len += 1;
        for _ in 0..n {
            cur_sum += ith(i);
            hor_len -= 1;
            i += hor_len;
        }

        res = res.max(cur_sum);
        beginning += hor_len0 + 2;
    }

    // down (left to right)
    beginning = 0;
    for len in (n..2 * n).rev() {
        let mut cur_sum = 0;

        let mut hor_len = n + 1;
        let mut i = beginning;
        for _ in 0..n {
            cur_sum += ith(i);
            i += hor_len;
            hor_len += 1;
        }
        i -= 1;
        hor_len -= 3;
        for _ in n..len {
            cur_sum += ith(i);
            i += hor_len;
            hor_len -= 1;
        }

        res = res.max(cur_sum);
        beginning += 1;
    }
    beginning = n;
    for (len, hor_len0) in (n..2 * n - 1).rev().zip(n + 1..) {
        let mut cur_sum = 0;

        let mut hor_len = hor_len0;
        let mut i = beginning;
        for _ in 0..len - n {
            cur_sum += ith(i);
            hor_len += 1;
            i += hor_len;
        }
        hor_len += 1;
        for _ in 0..n {
            cur_sum += ith(i);
            hor_len -= 1;
            i += hor_len;
        }

        res = res.max(cur_sum);
        beginning += hor_len0;
    }

    res
}
