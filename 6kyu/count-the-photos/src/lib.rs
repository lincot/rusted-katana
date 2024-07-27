//! <https://www.codewars.com/kata/6319dba6d6e2160015a842ed/train/rust>

pub const fn count_photos(road: &str) -> usize {
    let mut n_eastbound = 0;
    let mut n_cameras = 0;
    let mut photos = 0;
    let mut i = 0;
    while i < road.len() {
        match road.as_bytes()[i] {
            b'>' => n_eastbound += 1,
            b'<' => photos += n_cameras,
            _ => {
                n_cameras += 1;
                photos += n_eastbound;
            }
        }
        i += 1;
    }
    photos
}
