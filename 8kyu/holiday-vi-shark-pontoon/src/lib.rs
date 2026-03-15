//! <https://www.codewars.com/kata/57e921d8b36340f1fd000059/train/rust>

pub fn shark(
    pontoon_distance: f64,
    shark_distance: f64,
    you_speed: f64,
    shark_speed: f64,
    dolphin: bool,
) -> String {
    if pontoon_distance / you_speed < shark_distance * if dolphin { 2. } else { 1. } / shark_speed {
        "Alive!".into()
    } else {
        "Shark Bait!".into()
    }
}
