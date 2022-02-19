//! <https://www.codewars.com/kata/563b662a59afc2b5120000c6/train/rust>

pub fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let percent = percent * 0.01;
    let mut population = p0;
    let mut years = 0;

    while population < p {
        population += (percent * population as f64) as i32 + aug;
        years += 1;
    }

    years
}
