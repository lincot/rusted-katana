//! <https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/train/rust>

pub fn triangle(row_str: &str) -> String {
    let mut colors: Vec<_> = row_str
        .bytes()
        .map(|x| match x {
            b'R' => 0,
            b'G' => 1,
            b'B' => 2,
            _ => panic!(),
        })
        .collect();

    for row in 0..row_str.len() {
        for i in 0..colors.len() - 1 - row {
            if i + 1 >= row_str.len() {
                unsafe { core::hint::unreachable_unchecked() }
            }
            if colors[i] != colors[i + 1] {
                colors[i] = 3 - (colors[i] + colors[i + 1]);
            }
        }
    }

    match colors[0] {
        0 => "R",
        1 => "G",
        _ => "B",
    }
    .into()
}
