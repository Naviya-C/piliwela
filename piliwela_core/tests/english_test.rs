use piliwela_core::engine::english;

#[test]
fn test_preserve_python() {
    assert!(
        english::should_preserve(
            "Python"
        )
    );
}

#[test]
fn test_preserve_openai() {
    assert!(
        english::should_preserve(
            "OpenAI"
        )
    );
}

#[test]
fn test_single_a() {
    assert!(
        english::should_preserve(
            "a"
        )
    );
}

#[test]
fn test_single_f() {
    assert!(
        english::should_preserve(
            "f"
        )
    );
}