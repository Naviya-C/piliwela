use piliwela_core::{
    converter,
    mappings::common::FontFamily,
    options::ConvertOptions,
};

#[test]
fn test_single_conversion() {
    let options =
        ConvertOptions::default();

    assert_eq!(
        converter::convert(
            "wWu",
            FontFamily::FM,
            &options,
        ),
        "අඋම"
    );
}

#[test]
fn test_combo_conversion() {
    let options =
        ConvertOptions::default();

    assert_eq!(
        converter::convert(
            "fla",
            FontFamily::FM,
            &options,
        ),
        "කේ"
    );
}

#[test]
fn test_mixed() {
    let options =
        ConvertOptions::default();

    assert_eq!(
        converter::convert(
            "Python w",
            FontFamily::FM,
            &options,
        ),
        "Python අ"
    );
}