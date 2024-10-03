use counting_duplicates::count_duplicates;

#[test]
fn test() {
    assert_eq!(count_duplicates("indivisibility"), 1);
    assert_eq!(count_duplicates("индивизибилити"), 1);
    assert_eq!(count_duplicates("İndİvİsİbİlİty"), 1);
}
