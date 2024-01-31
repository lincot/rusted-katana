//! <https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust>

pub fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / (height * height);
    if bmi <= 18.5 {
        "Underweight"
    } else if bmi <= 25. {
        "Normal"
    } else if bmi <= 30. {
        "Overweight"
    } else {
        "Obese"
    }
}
