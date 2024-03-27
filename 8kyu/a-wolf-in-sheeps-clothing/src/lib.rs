//! <https://www.codewars.com/kata/5c8bfa44b9d1192e1ebd3d15/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_core::PushStrUnchecked;

pub fn warn_the_sheep(queue: &[&str]) -> String {
    let wolf_pos = queue
        .iter()
        .position(|x| x.bytes().next() == Some(b'w'))
        .unwrap();

    match queue.len() - wolf_pos - 1 {
        0 => "Pls go away and stop eating my sheep".into(),
        n => unsafe {
            let mut res = String::with_capacity(
                "Oi! Sheep number ! You are about to be eaten by a wolf!".len()
                    + usize::MAX_LEN_BASE10,
            );
            res.push_str_unchecked("Oi! Sheep number ");
            res.write_num_unchecked(n, 10, false, false);
            res.push_str_unchecked("! You are about to be eaten by a wolf!");
            res
        },
    }
}
