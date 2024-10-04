use its_raining_tacos::rain_tacos;

#[test]
fn test() {
    assert_eq!(
        rain_tacos("       \n       \n   Д   \n  ДДД  \n ТАКОС "),
        "       \n   O   \n  CДT  \n AДДДA \nTТАКОСC"
    );
}

#[test]
fn test_handles_bad_str() {
    let _ = rain_tacos("       \n       \n   Д   \n  ДДД  \n ТАК");
}
