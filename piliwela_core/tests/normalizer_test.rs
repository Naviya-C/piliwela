use piliwela_core::normalizer;

#[test]
fn test_nfc() {
    let result =
        normalizer::normalize(
            "කෙ"
        );

    assert_eq!(
        result,
        "කෙ"
    );
}

#[test]
fn test_whitespace_cleanup() {
    let result =
        normalizer::normalize(
            "a   b"
        );

    assert_eq!(
        result,
        "a b"
    );
}