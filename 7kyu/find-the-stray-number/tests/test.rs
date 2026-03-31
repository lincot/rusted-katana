use core::array;

use find_the_stray_number::stray;

#[test]
fn test() {
    let mut arr: [_; 1023] = array::from_fn(|_| 5);
    arr[arr.len() / 2] = 67;
    assert_eq!(stray(&arr), 67);

    assert_eq!(stray(&[1, 1, 1, 1, 1, 1, 2]), 2);
    assert_eq!(stray(&[2, 3, 2, 2, 2]), 3);
    assert_eq!(stray(&[3, 2, 2, 2, 2]), 3);
}
