//! <https://www.codewars.com/kata/57e8ff073d1cb559280005de/train/rust>

pub fn leg_room(a: u32, b: &str) -> &'static str {
    if b.contains('0') {
        return "Jackpot!";
    }
    let room = 2 * b
        .as_bytes()
        .iter()
        .filter(|b| !b"aeiou".contains(b))
        .count() as u32;
    let leg_length = a * 55 / 100;
    if room > leg_length / 4 {
        "super comfy"
    } else if room > leg_length * 3 / 20 {
        "comfortable"
    } else {
        "ouch"
    }
}
