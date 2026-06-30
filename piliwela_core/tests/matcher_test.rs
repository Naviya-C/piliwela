use piliwela_core::{
    engine::matcher,
    mappings::fm::FM_MAPPING,
};

#[test]
fn test_single() {
    assert_eq!(
        matcher::apply_combos_and_singles(
            "w",
            &FM_MAPPING,
        ),
        "අ"
    );
}

#[test]
fn test_combo() {
    assert_eq!(
        matcher::apply_combos_and_singles(
            "fla",
            &FM_MAPPING,
        ),
        "කේ"
    );
}

#[test]
fn test_longest_combo() {
    assert_eq!(
        matcher::apply_combos_and_singles(
            "flda",
            &FM_MAPPING,
        ),
        "කෝ"
    );
}

#[test]
fn test_unknown_char() {
    assert_eq!(
        matcher::apply_combos_and_singles(
            "123",
            &FM_MAPPING,
        ),
        "123"
    );
}