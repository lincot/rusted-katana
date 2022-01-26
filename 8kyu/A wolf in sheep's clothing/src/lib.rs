//! <https://www.codewars.com/kata/5c8bfa44b9d1192e1ebd3d15/train/rust>

pub fn warn_the_sheep(queue: &[&str]) -> String {
    let wolf_pos = queue
        .iter()
        .position(|x| x.bytes().next() == Some(b'w'))
        .unwrap();

    match queue.len() - wolf_pos - 1 {
        0 => String::from("Pls go away and stop eating my sheep"),
        n => format!("Oi! Sheep number {n}! You are about to be eaten by a wolf!"),
    }
}
