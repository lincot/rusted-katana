//! <https://www.codewars.com/kata/62b3e38df90eac002c7a983f/train/rust>

use unchecked::PushUnchecked;

pub fn solve(s: &str) -> String {
    const BACKSPACE: &[u8] = b"[backspace]";
    let s = s.as_bytes();
    let mut res = String::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with(BACKSPACE) {
            let mut multiplied = false;
            if let (Some(&b'*'), Some(rest)) =
                (s.get(i + BACKSPACE.len()), s.get(i + BACKSPACE.len() + 1..))
            {
                let num_end = rest
                    .iter()
                    .position(|x| !x.is_ascii_digit())
                    .unwrap_or(rest.len());
                if num_end != 0 {
                    let num = unsafe { rest.get_unchecked(..num_end) }
                        .iter()
                        .fold(0, |acc, &d| 10 * acc + (d - b'0') as u64);
                    for _ in 0..num {
                        res.pop();
                    }
                    i += BACKSPACE.len() + 1 + num_end;
                    multiplied = true;
                }
            }
            if !multiplied {
                res.pop();
                i += BACKSPACE.len();
            }
        } else {
            unsafe { res.as_mut_vec().push_unchecked(s[i]) };
            i += 1;
        }
    }
    res
}
