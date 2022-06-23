//! <https://www.codewars.com/kata/59f08f89a5e129c543000069/train/rust>

pub fn dup(mut arry: Vec<String>) -> Vec<String> {
    for s in &mut arry {
        let mut res = String::with_capacity(s.len());

        let mut prev = if let Some(c) = s.chars().next() {
            c
        } else {
            continue;
        };
        res.push(prev);
        for c in s.chars() {
            if c != prev {
                res.push(c);
            }
            prev = c;
        }

        *s = res;
    }

    arry
}
