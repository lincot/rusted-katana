//! <https://www.codewars.com/kata/628df6b29070907ecb3c2d83/train/rust>

pub const fn run(speed: u32, time: u32) -> u32 {
    let t = (time + 1) / 2;
    speed * time
        + if speed <= 3 * t {
            speed * (speed + 3) / 6 + !speed.is_multiple_of(3) as u32
        } else {
            t * (2 * speed - 3 * t + 3) / 2
        }
}
