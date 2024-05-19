//! <https://www.codewars.com/kata/5518a860a73e708c0a000027/train/rust>

pub fn last_digit(list: &[u64]) -> u64 {
    match list {
        [] => 1,
        [_, tail @ ..] if evals_to_zero(tail) => 1,
        [a, tail @ ..] => {
            let exp_mod_4 = match tail {
                [] => 1,
                [_, tail @ ..] if evals_to_zero(tail) => 1,
                [b] => (b % 4) as u8,
                [b, c, tail @ ..] => match b % 4 {
                    0 => 0,
                    1 => 1,
                    2 => {
                        if *c == 1 || evals_to_zero(tail) {
                            2
                        } else {
                            0
                        }
                    }
                    3 => {
                        if c % 2 == 0 && !evals_to_zero(tail) {
                            1
                        } else {
                            3
                        }
                    }
                    _ => unreachable!(),
                },
            };
            TABLE[(a % 10) as usize][exp_mod_4 as usize] as _
        }
    }
}

const TABLE: [[u8; 4]; 10] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [6, 2, 4, 8],
    [1, 3, 9, 7],
    [6, 4, 6, 4],
    [5, 5, 5, 5],
    [6, 6, 6, 6],
    [1, 7, 9, 3],
    [6, 8, 4, 2],
    [1, 9, 1, 9],
];

fn evals_to_zero(list: &[u64]) -> bool {
    list.iter().take_while(|&&x| x == 0).count() % 2 == 1
}
