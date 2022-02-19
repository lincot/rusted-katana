//! <https://www.codewars.com/kata/605ae9e1d2be8a0023b494ed/train/rust>

pub fn count_salutes(hallway: &str) -> usize {
    let mut moving_right = 0;
    let mut salutes = 0;

    for guy in hallway.bytes() {
        match guy {
            b'>' => moving_right += 1,
            b'<' => salutes += moving_right,
            _ => {}
        }
    }

    2 * salutes
}
