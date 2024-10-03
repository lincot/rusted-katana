use sort_by_last_char::sort_by_last_char;

#[test]
fn test() {
    assert_eq!(sort_by_last_char("я в б"), vec!["б", "в", "я"]);
}
