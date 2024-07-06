//! <https://www.codewars.com/kata/568d0dd208ee69389d000016/train/rust>

pub const fn rental_car_cost(d: u32) -> u32 {
    40 * d
        - match d {
            7.. => 50,
            3.. => 20,
            _ => 0,
        }
}
