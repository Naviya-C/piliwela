use piliwela_core::{
    engine::{
        rules,
        MappingSet,
    },
    mappings::fm::FM_MAPPING,
};

#[test]
fn test_rule_percent_a() {
    assert_eq!(
        rules::apply_rules(
            "%a",
            &FM_MAPPING,
        ),
        "a%"
    );
}

#[test]
fn test_rule_a() {
    assert_eq!(
        rules::apply_rules(
            "A",
            &FM_MAPPING,
        ),
        "a"
    );
}