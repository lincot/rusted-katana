use dont_give_me_five::dont_give_me_five;

#[test]
fn test() {
    assert_eq!(dont_give_me_five(-100, -22), 62);
    assert_eq!(dont_give_me_five(0, 5), 5);
    assert_eq!(dont_give_me_five(-5, 0), 5);
}
