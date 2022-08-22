//! <https://www.codewars.com/kata/568dc014440f03b13900001d/train/rust>

pub fn get_drink_by_profession(param: &str) -> &'static str {
    match param.to_ascii_lowercase().as_str() {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer",
    }
}
