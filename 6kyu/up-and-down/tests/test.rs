use up_and_down::arrange;

#[test]
fn test() {
    assert_eq!(
        arrange("who hit retaining The That a we taken"),
        "who RETAINING hit THAT a THE we TAKEN"
    );
    assert_eq!(arrange("hİt"), "hi̇t");
}
