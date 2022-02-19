//! <https://www.codewars.com/kata/59c62f1bdcc40560a2000060/train/rust>

pub fn solve(v: &[String]) -> i32 {
    let mut balance = 0;

    for x in v {
        match x.bytes().last() {
            Some(b'0') => balance += 1,
            Some(b'1') => balance -= 1,
            Some(b'2') => balance += 1,
            Some(b'3') => balance -= 1,
            Some(b'4') => balance += 1,
            Some(b'5') => balance -= 1,
            Some(b'6') => balance += 1,
            Some(b'7') => balance -= 1,
            Some(b'8') => balance += 1,
            Some(b'9') => balance -= 1,
            _ => {}
        }
    }

    balance
}
