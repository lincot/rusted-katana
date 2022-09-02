//! <https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/train/rust>

pub fn triangle(row_str: &str) -> String {
    assert!(!row_str.is_empty());

    let mut colors = Vec::with_capacity(row_str.len());
    unsafe { colors.set_len(row_str.len()) };
    let mut colors_ptr = colors.as_mut_ptr();
    for x in row_str.bytes() {
        unsafe {
            *colors_ptr = match x {
                b'R' => 0,
                b'G' => 1,
                _ => 2,
            };
            colors_ptr = colors_ptr.add(1);
        }
    }

    for row in 0..colors.len() {
        for i in 0..colors.len() - row - 1 {
            if i + 1 >= colors.len() {
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
