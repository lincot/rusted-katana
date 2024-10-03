use word_to_initial_number::convert;

#[test]
fn test() {
    assert_eq!(convert("CodeWars"), 10_234_567);
    assert_eq!(convert("КодеВарс"), 10_234_567);
    assert_eq!(convert("KATA"), 1020);
    assert_eq!(convert("KİTİ"), 1020);
}
