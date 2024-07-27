//! <https://www.codewars.com/kata/59279aea8270cc30080000df/train/rust>

pub fn freeway_game(dist_km_to_exit: f64, my_speed_kph: f64, other_cars: &[(f64, f64)]) -> i32 {
    let my_time = dist_km_to_exit / my_speed_kph;
    let mut res = 0;
    for &(start_time, other_speed) in other_cars {
        let start_time = start_time * (1. / 60.);
        let other_time = dist_km_to_exit / other_speed + start_time;
        if start_time < 0. && other_time > my_time {
            res += 1;
        } else if start_time >= 0. && other_time < my_time {
            res -= 1;
        }
    }
    res
}
