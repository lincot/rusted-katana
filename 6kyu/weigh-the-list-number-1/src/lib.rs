//! <https://www.codewars.com/kata/5fad2310ff1ef6003291a951/train/rust>

#![feature(array_chunks)]

pub fn weigh_the_list(mut a: Vec<i64>) -> Vec<i64> {
    for [a, b] in a.array_chunks_mut() {
        (*a, *b) = (*b, -*a);
    }
    a
}
