//! <https://www.codewars.com/kata/5a53a17bfd56cb9c14000003/train/rust>

pub fn disarium_number(n: u32) -> String {
    if [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 89, 135, 175, 518, 598, 1306, 1676, 2427, 2646798,
    ]
    .contains(&n)
    {
        "Disarium !!"
    } else {
        "Not !!"
    }
    .into()
}
