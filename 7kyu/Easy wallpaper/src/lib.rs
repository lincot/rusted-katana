//! <https://www.codewars.com/kata/567501aec64b81e252000003/train/rust>

pub fn wall_paper(l: f64, w: f64, h: f64) -> String {
    if [l, w, h].contains(&0.) {
        return "zero".into();
    }

    match (1.15f64 * 2. / (0.52 * 10.)).mul_add(h * (l + w), 0.99999) as u8 {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => panic!(),
    }
    .into()
}
