//! <https://www.codewars.com/kata/62eb800ba29959001c07dfee/train/rust>

pub fn brightest(colors: &[String]) -> String {
    colors
        .iter()
        .rev()
        .max_by_key(|c| {
            let c = c.as_bytes();
            assert!(c.len() == 7);
            [c[1], c[2]].max([c[3], c[4]]).max([c[5], c[6]])
        })
        .unwrap()
        .clone()
}
