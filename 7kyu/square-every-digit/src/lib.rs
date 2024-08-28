//! <https://www.codewars.com/kata/546e2562b03326a88e000020/train/rust>

pub const fn square_digits(mut num: u64) -> u64 {
    let mut res = 0;
    let mut mul = 1;
    while num >= 10 {
        let (digits_squared, m) = TABLE[(num % 100) as usize];
        num /= 100;
        res += digits_squared as u64 * mul;
        mul *= m as u64;
    }
    if num != 0 {
        res += num.pow(2) * mul;
    }
    res
}

const TABLE: [(u32, u32); 100] = [
    (0, 100),
    (1, 100),
    (4, 100),
    (9, 100),
    (16, 1000),
    (25, 1000),
    (36, 1000),
    (49, 1000),
    (64, 1000),
    (81, 1000),
    (10, 100),
    (11, 100),
    (14, 100),
    (19, 100),
    (116, 1000),
    (125, 1000),
    (136, 1000),
    (149, 1000),
    (164, 1000),
    (181, 1000),
    (40, 100),
    (41, 100),
    (44, 100),
    (49, 100),
    (416, 1000),
    (425, 1000),
    (436, 1000),
    (449, 1000),
    (464, 1000),
    (481, 1000),
    (90, 100),
    (91, 100),
    (94, 100),
    (99, 100),
    (916, 1000),
    (925, 1000),
    (936, 1000),
    (949, 1000),
    (964, 1000),
    (981, 1000),
    (160, 1000),
    (161, 1000),
    (164, 1000),
    (169, 1000),
    (1616, 10000),
    (1625, 10000),
    (1636, 10000),
    (1649, 10000),
    (1664, 10000),
    (1681, 10000),
    (250, 1000),
    (251, 1000),
    (254, 1000),
    (259, 1000),
    (2516, 10000),
    (2525, 10000),
    (2536, 10000),
    (2549, 10000),
    (2564, 10000),
    (2581, 10000),
    (360, 1000),
    (361, 1000),
    (364, 1000),
    (369, 1000),
    (3616, 10000),
    (3625, 10000),
    (3636, 10000),
    (3649, 10000),
    (3664, 10000),
    (3681, 10000),
    (490, 1000),
    (491, 1000),
    (494, 1000),
    (499, 1000),
    (4916, 10000),
    (4925, 10000),
    (4936, 10000),
    (4949, 10000),
    (4964, 10000),
    (4981, 10000),
    (640, 1000),
    (641, 1000),
    (644, 1000),
    (649, 1000),
    (6416, 10000),
    (6425, 10000),
    (6436, 10000),
    (6449, 10000),
    (6464, 10000),
    (6481, 10000),
    (810, 1000),
    (811, 1000),
    (814, 1000),
    (819, 1000),
    (8116, 10000),
    (8125, 10000),
    (8136, 10000),
    (8149, 10000),
    (8164, 10000),
    (8181, 10000),
];
