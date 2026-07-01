use piliwela_core::{
    detector,
    mappings::common::FontFamily,
};

#[test] 
fn test_detect_fm() {
    assert_eq!(
        detector::detect("wWu fla"),
        FontFamily::FM,
    ); 
}

#[test]
fn test_detect_unknown() {
    assert_eq!(
        detector::detect("Hello World"),
        FontFamily::Unknown,
    );
}

#[test]
fn test_detect_numbers() {
    assert_eq!(
        detector::detect("123456"),
        FontFamily::Unknown,
    );
}

#[test]
fn test_detect_empty() {
    assert_eq!(
        detector::detect(""),
        FontFamily::Unknown,
    );
}