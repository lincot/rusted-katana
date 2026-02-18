//! <https://www.codewars.com/kata/659af96994b858db10e1675f/train/rust>

pub fn find(s: &str) -> u32 {
    const MAX_U32_LEN: usize = 10;

    let bytes = s.as_bytes();
    let mut num = [b'0'; MAX_U32_LEN + 1];

    for starting_len in 1..bytes.len().min(10) {
        num[MAX_U32_LEN + 1 - starting_len..].copy_from_slice(&bytes[..starting_len]);
        let mut num_len = starting_len;
        let mut num_start = starting_len;

        loop {
            increment_num_str(&mut num, &mut num_len);
            if num_len > num.len() {
                break;
            }

            let Some(next_num) = bytes.get(num_start..num_start + num_len) else {
                break;
            };

            if next_num != &num[num.len() - num_len..] {
                break;
            }

            num_start += num_len;

            if num_start == bytes.len() {
                return s[..starting_len].parse().unwrap();
            }
        }
    }

    s.parse().unwrap()
}

fn increment_num_str(num: &mut [u8], num_len: &mut usize) {
    let mut increment_num_len = false;
    for (i, digit) in num.iter_mut().rev().enumerate() {
        if *digit == b'9' {
            *digit = b'0';
            increment_num_len = true;
        } else {
            *digit += 1;

            if i == *num_len && increment_num_len {
                *num_len += 1;
            }

            return;
        }
    }
}
