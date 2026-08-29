use the_latest_clock::latest_clock;

#[test]
fn test() {
    assert_eq!(latest_clock(9, 9, 2, 0), "09:29");
    assert_eq!(latest_clock(8, 4, 1, 8), "18:48");
    assert_eq!(latest_clock(5, 6, 1, 9), "19:56");
    assert_eq!(latest_clock(0, 8, 8, 3), "08:38");
    assert_eq!(latest_clock(6, 2, 0, 9), "09:26");
}
