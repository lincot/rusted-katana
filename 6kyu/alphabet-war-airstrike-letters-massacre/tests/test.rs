use core::{array, cmp::Ordering};

use alphabet_war_airstrike_letters_massacre::alphabet_war;
use rand::seq::IndexedRandom;
use rand_pcg::Pcg64Mcg;

#[test]
fn test() {
    let fight = "aaaaaaaaaaaaaaaammmmmmmmmmmmmmmm";
    assert_eq!(alphabet_war(fight), alphabet_war_naive(fight));

    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let fight: [_; if cfg!(miri) { 16 } else { 1000 }] =
        array::from_fn(|_| *b"wpbsmqdz*".choose(&mut rng).unwrap());
    let fight = unsafe { core::str::from_utf8_unchecked(&fight) };

    assert_eq!(alphabet_war(fight), alphabet_war_naive(fight));
}

pub fn alphabet_war_naive(fight: &str) -> &'static str {
    let fight = fight.as_bytes();

    let mut i = 0;
    let mut score = 0;
    let mut prev_power = 0;
    let mut skip_next = false;
    while i < fight.len() {
        let b = fight[i];

        if b == b'*' {
            score -= prev_power;
            prev_power = 0;
            skip_next = true;
            i += 1;
            continue;
        }

        let power = match b {
            b'w' => -4isize,
            b'p' => -3,
            b'b' => -2,
            b's' => -1,
            b'm' => 4,
            b'q' => 3,
            b'd' => 2,
            b'z' => 1,
            _ => 0,
        };
        prev_power = if skip_next { 0 } else { power };
        if !skip_next {
            score += power;
        }
        skip_next = false;

        i += 1;
    }

    match score.cmp(&0) {
        Ordering::Less => "Left side wins!",
        Ordering::Equal => "Let's fight again!",
        Ordering::Greater => "Right side wins!",
    }
}
