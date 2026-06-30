use piliwela_core::engine::kombuva;

#[test]
fn test_fix_kombuva() {
    assert_eq!(
        kombuva::fix_kombuva(
            "ෙක"
        ),
        "කෙ"
    );
}

#[test]
fn test_no_change() {
    assert_eq!(
        kombuva::fix_kombuva(
            "කම"
        ),
        "කම"
    );
}