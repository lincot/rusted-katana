//! <https://www.codewars.com/kata/5fad2310ff1ef6003291a951/train/rust>

pub fn weigh_the_list(mut a: Vec<i64>) -> Vec<i64> {
    for chunk in a.chunks_exact_mut(2) {
        (chunk[0], chunk[1]) = (chunk[1], -chunk[0]);
    }
    a
}
