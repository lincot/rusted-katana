//! <https://www.codewars.com/kata/564d0490e96393fc5c000029/train/rust>

pub const fn coin_combo(cents: u64) -> [u64; 4] {
    let (quarters, rest) = (cents / 25, cents % 25);
    let (dimes, rest) = (rest / 10, rest % 10);
    let (nickel, pennies) = (rest / 5, rest % 5);
    [pennies, nickel, dimes, quarters]
}
