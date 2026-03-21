use alternate_capitalization::capitalize;

#[test]
fn test() {
    assert_eq!(capitalize("абракадабра"), ["АбРаКаДаБрА", "аБрАкАдАбРа"]);
}
