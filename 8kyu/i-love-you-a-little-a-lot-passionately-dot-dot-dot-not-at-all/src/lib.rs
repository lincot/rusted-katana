//! <https://www.codewars.com/kata/57f24e6a18e9fad8eb000296/train/rust>

#![no_std]

pub const fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    match nb_petals % 6 {
        0 => "not at all",
        1 => "I love you",
        2 => "a little",
        3 => "a lot",
        4 => "passionately",
        5 => "madly",
        _ => unreachable!(),
    }
}
