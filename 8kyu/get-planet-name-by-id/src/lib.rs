//! <https://www.codewars.com/kata/515e188a311df01cba000003/train/rust>

pub fn get_planet_name(id: u32) -> String {
    match id {
        1 => "Mercury",
        2 => "Venus",
        3 => "Earth",
        4 => "Mars",
        5 => "Jupiter",
        6 => "Saturn",
        7 => "Uranus",
        8 => "Neptune",
        _ => panic!(),
    }
    .into()
}
