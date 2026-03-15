use get_the_middle_character::get_middle;

#[test]
fn test() {
    assert_eq!(get_middle("алиса"), "и");
    assert_eq!(get_middle("алис"), "ли");
    assert_eq!(get_middle("алллvllис"), "v");
}
