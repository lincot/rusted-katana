//! <https://www.codewars.com/kata/60576b180aef19001bce494d/train/rust>

pub fn count_checkerboard(width: u128, height: u128, resolution: u128) -> u128 {
    let (dx, dy) = (width % (2 * resolution), height % (2 * resolution));
    let square = (width * height - dx * dy) / 2;
    let (ddx, ddy) = (dx % (2 * resolution), dy % (2 * resolution));
    let edges = (ddx - resolution.min(ddx)) * resolution.min(ddy)
        + (ddy - resolution.min(ddy)) * resolution.min(ddx);
    square + edges
}
