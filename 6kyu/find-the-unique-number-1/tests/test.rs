use core::array;

use find_the_unique_number_1::find_uniq;

#[test]
fn test() {
    let mut arr: [_; 1023] = array::from_fn(|_| 5.);
    arr[arr.len() / 2] = 67.;
    assert_eq!(find_uniq(&arr), 67.);

    assert_eq!(find_uniq(&[1., 1., 1., 1., 1., 1., 2.]), 2.);
    assert_eq!(find_uniq(&[2., 3., 2., 2., 2.]), 3.);
    assert_eq!(find_uniq(&[3., 2., 2., 2., 2.]), 3.);
}
